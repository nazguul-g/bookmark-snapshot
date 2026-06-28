# Code Review: Bookmark Tree

A senior engineer's walkthrough of every problem in this project, from types to
file placement. Read this top-to-bottom, then fix in the suggested order.

---

## 1. Root-Cause Mistake: `#![allow(unused)]`

**File:** `src/main.rs:1`

```rusts
#![allow(unused)]
```

This single attribute is a **red flag**. It hides every compiler warning about
dead code, unused imports, and unused variables. Without it, the compiler would
have screamed about:

- The dead `JSONBookmarkSchema` struct in `chromium.rs`
- Unnecessary imports throughout the codebase
- `validate_schema()` being a no-op
- Unused `Hash` derives

**Fix:** Delete this line and fix every warning the compiler produces.

---

## 2. Type System Issues

### 2.1 `Browser::userdata_path` is a HashMap when it should be a single path

**File:** `src/types.rs:6`

```rust
pub userdata_path: HashMap<SupportedOSs, String>,
```

Every `Browser` is created at runtime for ONE browser on ONE OS. A HashMap of
OS→path is over-engineered: you populate paths for both Linux and Windows, but
only one is ever used. It also infects every function that touches the path
(`pattern_builder`, `glob_search_bookmarks_path`) with a
`.get(&SupportedOSs::Linux).unwrap()`.

**Fix:** Replace with a single `String` (or `PathBuf`). Only store the current
OS's path. If you need cross-platform, store both in the type that represents
configuration, not in the runtime `Browser` struct.

```rust
pub struct Browser {
    pub name: SupportedBrowsers,
    pub userdata_path: String,     // single path for current OS
    pub store_type: BookmarkStoreType,
    pub bookmark_path: Option<PathBuf>,
}
```

### 2.2 `CliOptions::supported_os` is `Option<SupportedOSs>` but is always required after init

**File:** `src/types.rs:114`

```rust
pub supported_os: Option<SupportedOSs>,
```

It starts as `None` in `new()`, but by the time it reaches `search_browsers()`,
it's expected to be `Some(...)`. The `None` match arm uses `unreachable!()`.
An `Option` whose `None` case is unreachable by design should not be `Option`.

**Fix:** Either make it `SupportedOSs` and require it at construction, or use
a builder pattern:

```rust
pub struct CliOptions {
    pub browsers: Vec<Browser>,
    pub routine: Option<Routine>,
    pub save_path: Option<String>,
    pub routine_count: u32,
    pub supported_os: SupportedOSs,   // no Option
}
```

### 2.3 `BookmarkStoreType` and `Routine` have unnecessary derives

**File:** `src/types.rs:90-106`

```rust
#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize, JsonSchema)]
pub enum BookmarkStoreType { JSON, SQLite }
```

`Hash` is derived on three types (`BookmarkStoreType`, `Routine`, `SupportedOSs`
and `BookmarkNode` et al. in `parser/types.rs`) but **never used as a hash key**.
Unnecessary derives add noise and constrain what fields you can add later.

**Guideline:** Only derive what you actually use. `Debug, Clone, PartialEq` are
always safe. `Serialize, Deserialize` if you JSON-serialize. Add `Hash` only
when you actually put it in a `HashSet` or `HashMap` key.

### 2.4 `BookmarkNode` in `parser/types.rs` derives `Hash` — but contains `String` fields

**File:** `src/parser/types.rs:22`

This works because `String` implements `Hash`, but do you actually hash these
nodes? No. Unnecessary derives.

### 2.5 Dead code: `JSONBookmarkSchema`

**File:** `src/parser/chromium.rs:1-3`

```rust
#[derive(Serialize, Deserialize)]
pub struct JSONBookmarkSchema {}
```

Defined at the top of the file (before imports!) and never used. It's a relic.

**Fix:** Delete it.

### 2.6 `IO::Error` from `systemd` crate is unused

**File:** `src/schedular/linux.rs:9`

```rust
use crate::io::{self, ...};
```

`io::` is never used as a module path in this file. The import is dead.

---

## 3. Function Signature Problems

### 3.1 Inconsistent path types: `&str` vs `&Path` vs `PathBuf`

The codebase can't decide how to represent paths:

| Function | File | Path Type |
|---|---|---|
| `check_path` | `io/browsers.rs` | `&str` |
| `get_home_directory` | `io/browsers.rs` | returns `String` |
| `request_path` | `io/browsers.rs` | returns `String` |
| `pattern_builder` | `io/browsers.rs` | `&str` |
| `read_file` | `parser/chromium.rs` | `&str` |
| `write_file` | `parser/chromium.rs` | `&Path` |
| `json_parser` | `parser/chromium.rs` | `&str` for directory |

**Rule:** Use `PathBuf` for owned paths and `&Path` for borrowed paths. Never
use `String` for something that represents a filesystem path. The `Path` type
is cross-platform, has `.join()`, `.parent()`, `.exists()`, etc. — use it.

**Fix example:**

```rust
// Instead of this:
pub fn get_home_directory() -> String { ... }
pub fn check_path(path: &str) -> bool { ... }

// Do this:
pub fn get_home_directory() -> PathBuf { ... }
pub fn check_path(path: &Path) -> bool { path.exists() }
```

### 3.2 `fn cli() -> Result<(), Box<dyn Error>>` — the return value is never checked

**File:** `src/cli.rs:42`

The function returns `Result`, but inside it calls `.exit()` on parse errors
and `exit(1)` on other failures. It also uses `unwrap()` liberally. The
`Result` is a lie — the function either panics or exits the process.

`main.rs` calls `cli();` without checking the result:

```rust
fn main() {
    cli();
}
```

**Fix:** Either:
- Remove the `Result` return and use `exit()`/`panic!` everywhere (current
  style, but make it consistent), OR
- Propagate all errors via `?` and handle them in `main()`:

```rust
fn main() -> Result<(), Box<dyn Error>> {
    cli()?;
    Ok(())
}
```

Pick one. Don't mix both.

### 3.3 `search_browsers(options: &CliOptions)` clones unnecessarily

**File:** `src/io/browsers.rs:29`

```rust
pub fn search_browsers(options: &CliOptions) -> Result<CliOptions, Box<dyn Error>> {
    let mut browsers = options.browsers.clone();
    // ... modify browsers ...
    let mut options = options.clone();  // second clone!
    options.browsers = browsers;
    Ok(options)
}
```

Two clones of potentially large data. Caller in `cli.rs` already owns the
`options`:

```rust
let config = if let Ok(config) = search_browsers(&options) {
    config
};
```

Fix the signature to take ownership when you need ownership:

```rust
pub fn search_browsers(mut options: CliOptions) -> Result<CliOptions, Box<dyn Error>> {
    for b in &mut options.browsers {
        let path = glob_search_bookmarks_path(b)?;
        b.bookmark_path = Some(path);
    }
    Ok(options)
}
```

### 3.4 `glob_search_bookmarks_path(browser: &mut Browser)` mutates without needing to

**File:** `src/io/browsers.rs:58`

```rust
fn glob_search_bookmarks_path(browser: &mut Browser) -> Option<PathBuf> {
```

It takes `&mut Browser` but doesn't mutate it — it returns the path and lets
the caller assign it. The `&mut` is only needed because of the
`request_path(&browser.name)` call, which takes a `&SupportedBrowsers`.

**Fix:** Take `&Browser` (immutable reference) and return `Option<PathBuf>`.
Let the caller assign `b.bookmark_path = path`.

### 3.5 `request_path(name: &SupportedBrowsers)` takes a reference to a `Copy` type

**File:** `src/io/browsers.rs:93`

`SupportedBrowsers` derives `Clone` and has no heap data. Tiny enums like this
should be passed by value:

```rust
pub fn request_path(name: SupportedBrowsers) -> PathBuf {
```

### 3.6 `pattern_builder(userdata: &str, store_type: &BookmarkStoreType)` takes reference to `Copy` type

**File:** `src/io/browsers.rs:113`

`BookmarkStoreType` is a fieldless enum — it's `Copy`. Take it by value:

```rust
fn pattern_builder(userdata: &str, store_type: BookmarkStoreType) -> String {
```

### 3.7 `save_config_linux(cli_options: &CliOptions)` clones at the top

**File:** `src/io/config.rs:20`

```rust
pub fn save_config_linux(cli_options: &CliOptions) -> Result<(), Box<dyn Error>> {
    let mut options = cli_options.clone();
    let save_path = if let Some(path) = options.save_path { ... };
    options.save_path = Some(save_path);
```

This clones the entire `CliOptions` (which contains `Vec<Browser>` with all
their data) just to set `save_path` to its default. But `save_path` is already
set in `handle_matches()` before this is called! The clone is completely
unnecessary.

**Fix:** Take `&CliOptions`, don't clone. Trust that the caller has already
set `save_path`. Or use a builder pattern.

### 3.8 `render_timer_unit() -> Result<(String), Box<dyn Error>>` has useless parentheses

**File:** `src/schedular/linux.rs:19`

```rust
fn render_timer_unit() -> Result<(String), Box<dyn Error>> {
```

`(String)` is just `String`. The parentheses do nothing.

**Fix:**

```rust
fn render_timer_unit() -> Result<String, Box<dyn Error>> {
```

### 3.9 `generate_name(browser: &Browser)` takes a `Browser` but only uses `browser.name`

**File:** `src/parser/chromium.rs:70`

```rust
fn generate_name(browser: &Browser) -> String {
    let file_name = format!("{}_snapshot{}.json", browser.name, timestamp);
```

The function only needs the name. Taking the whole `Browser` creates an
unnecessary dependency and makes the function harder to test.

**Fix:**

```rust
fn generate_name(browser_name: &SupportedBrowsers) -> String {
```

Or even simpler, take `&dyn Display` or a `&str`.

### 3.10 `read_file` and `write_file` have inconsistent signatures

**File:** `src/parser/chromium.rs:78-88`

```rust
fn read_file(path: &str) -> Result<BufReader<File>, Box<dyn Error>>
fn write_file(path: &Path) -> Result<BufWriter<File>, Box<dyn Error>>
```

One takes `&str`, the other `&Path`. Pick one convention. `&Path` is preferred.

---

## 4. Error Handling (The Biggest Problem)

The codebase uses **at least 5 different error handling strategies**,
sometimes in the same function:

| Strategy | Used in |
|---|---|
| `panic!` / `unwrap()` | `config.rs`, `browsers.rs`, `chromium.rs` |
| `exit(1)` | `cli.rs`, `browsers.rs` |
| `eprintln!` then `exit` | `cli.rs`, `browsers.rs` |
| `todo!()` | `browsers.rs` (Windows branch) |
| `?` operator | `chromium.rs`, `linux.rs`, `config.rs` |

### 4.1 Silently ignored `Result`

**File:** `src/io/config.rs:45`

```rust
serde_json::to_writer_pretty(writer, &options);
```

If serialization fails, this error is **discarded silently**. The function
continues and prints "config saved". This is a bug.

**Fix:**

```rust
serde_json::to_writer_pretty(writer, &options)?;
```

### 4.2 `unwrap()` on directory creation

**File:** `src/io/config.rs:37`

```rust
create_dir(&config_path).unwrap();
```

If the directory can't be created, the program panics. Use `?` instead.

### 4.3 `Box<dyn Error>` everywhere

Every function that returns `Result` uses `Box<dyn Error>`. This is fine for
prototyping, but it means callers can never match on specific errors. Consider
a custom error enum:

```rust
#[derive(Debug)]
pub enum AppError {
    ConfigNotFound,
    BrowserNotFound(String),
    Io(std::io::Error),
    Serde(serde_json::Error),
    UnsupportedOS(String),
    Unimplemented(String),
}

impl From<std::io::Error> for AppError { ... }
impl From<serde_json::Error> for AppError { ... }
// etc.
```

### 4.4 Error in `linux.rs` swallows the original error

**File:** `src/schedular/linux.rs:56-61`

```rust
let timer_unit = if let Ok(unit) = render_timer_unit() {
    unit
} else {
    return Err(std::io::Error::new(NotFound, "not routine found"));
};
```

If `render_timer_unit()` fails for **any reason** (config corruption, IO error,
etc.), the original error is thrown away and replaced with a generic "not
routine found" message. This makes debugging impossible.

**Fix:**

```rust
let timer_unit = render_timer_unit()
    .map_err(|e| std::io::Error::new(NotFound, format!("failed to render timer: {e}")))?;
```

### 4.5 `todo!()` in production code

**File:** `src/io/browsers.rs:38`

```rust
Some(SupportedOSs::Windows) => todo!(),
```

`todo!()` panics at runtime. If someone runs this on Windows, the program
crashes with no helpful message. Use `unimplemented!()` or return an `Err`:

```rust
Some(SupportedOSs::Windows) => {
    return Err("Windows support not yet implemented".into());
}
```

---

## 5. File and Function Placement Issues

### 5.1 `sqlite_parser` lives in `chromium.rs` — wrong module

**File:** `src/parser/chromium.rs:66`

```rust
fn sqlite_parser(browser: &Browser, save_directory: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}
```

This function exists to handle **Firefox** bookmarks (which use SQLite), but
it's placed in the Chromium parser module. It should be in `parser/gecko.rs`,
or at least renamed to something that clarifies it's not Chromium-related.

**Current dispatch logic in `chromium.rs:39-42`:**

```rust
match b.store_type {
    BookmarkStoreType::JSON => json_parser(b, &save_path)?,
    BookmarkStoreType::SQLite => sqlite_parser(b, &save_path)?,
}
```

The dispatch is based on store type, not browser name. This means if you add a
new browser that uses JSON (e.g., Edge), it works. But the SQLite handler for
Firefox lives in the wrong file.

**Fix:** Move `sqlite_parser` into `gecko.rs` and dispatch there.

### 5.2 `check_path`, `get_home_directory`, `get_input` are generic utilities, not browser-specific

**File:** `src/io/browsers.rs`

These functions are used across modules (by `config.rs`, `chromium.rs`,
`linux.rs`), yet they live in `io/browsers.rs`. They should be in a separate
utility module or directly in `io/mod.rs`.

**Fix:** Create `src/io/utils.rs` or `src/util.rs`:

```rust
// src/io/utils.rs
pub fn get_home_directory() -> PathBuf { ... }
pub fn get_input(message: &str) -> String { ... }
// check_path is just path.exists() — remove it entirely
```

### 5.3 `cli.rs` is both the CLI parser AND the application controller

**File:** `src/cli.rs:109-184`

`handle_matches()` does:
1. Parse OS
2. Map browser strings to Browser structs
3. Parse routine options
4. Validate output path
5. Search for browsers on disk
6. Save config
7. Take snapshot
8. Set up schedule

This is **too much responsibility** for one function. `cli.rs` should parse
arguments and return a `CliOptions`. The orchestration belongs in `main.rs` or
a separate `app.rs`.

**Fix:**

```rust
// cli.rs — ONLY argument parsing
pub fn parse_args() -> Result<CliOptions, Box<dyn Error>> { ... }

// main.rs — orchestration
fn main() -> Result<(), Box<dyn Error>> {
    let opts = cli::parse_args()?;
    let opts = search_browsers(opts)?;
    save_config(&opts)?;
    snapshot(&opts)?;
    schedule(&opts)?;
    Ok(())
}
```

### 5.4 `chromium.rs` is named after a browser but handles the general JSON case

The module is `parser::chromium`, but its main function `snapshot()` handles
**all** browsers (reads config, dispatches by store type). The Chromium-specific
logic is only `json_parser`. Future browsers that use JSON (Edge, Vivaldi,
Opera) should also use `json_parser`, but the module name suggests it's
Chromium-only.

**Consider:** Rename to `parser::json` or split: move `snapshot()` to a higher
level, keep `json_parser` in a `parser::json` module.

### 5.5 The `schedular` directory is misspelled

It should be `scheduler`. This is a directory name, a module name, and appears
in `mod` declarations and `use` paths. Misspelling it now will cause confusion
forever.

**Fix:**

```bash
mv src/schedular src/scheduler
```

And update all references.

---

## 6. Logic and Design Issues

### 6.1 `Routine::Month` uses `count * 30` days — inaccurate

**File:** `src/schedular/linux.rs:35-37`

```rust
Routine::Month => {
    format!("{}d", count * 30)
}
```

Months are not all 30 days. Over time, this drifts. Use `OnCalendar` with
monthly syntax instead of `OnUnitActiveSec`:

```
OnCalendar=monthly
```

### 6.2 The systemd service file has a hardcoded dev path

**File:** `assets/bookmark-tree.service`

```
ExecStart=/mnt/workspace/Projects/bookmark-tree/target/debug/bookmark-tree --outputpath /mnt/downloads
```

This path is embedded at compile time via `include_str!`. It will never work
on another machine. The service unit should be generated dynamically, like the
timer unit is.

### 6.3 Recursive path validation in `request_path` can stack overflow

**File:** `src/io/browsers.rs:93-112`

```rust
pub fn request_path(name: &SupportedBrowsers) -> String {
    // ...
    if check_path(&path) {
        return path;
    } else {
        request_path(name)  // recursion!
    }
}
```

If a user keeps entering invalid paths, this recurses infinitely until stack
overflow. Use a loop:

```rust
pub fn request_path(name: &SupportedBrowsers) -> String {
    loop {
        // ...
        if check_path(&path) {
            return path;
        }
        eprintln!("Path does not exist, try again");
    }
}
```

### 6.4 `check_path` is a one-liner wrapper around `Path::exists()`

**File:** `src/io/browsers.rs:13-19`

```rust
pub fn check_path(path: &str) -> bool {
    let path = Path::new(path);
    if path.exists() {
        return true;
    }
    false
}
```

This is 7 lines for `Path::new(path).exists()`. Inline it everywhere.

### 6.5 `Main.rs` has an unused `use` (not really, but check)

Actually, `main.rs` doesn't have `use crate::cli::cli;` — wait, it does on line
3. That's used. OK. But the module-level modules `io`, `types`, `parser`,
`schedular` are declared but `io`, `types`, and `parser` are only used
indirectly by `cli`. The `mod` declarations in main are correct.

### 6.6 No consistent logging

Some messages use `println!`, some use `eprintln!`, some use `colored` for
styling, some don't. Introduce a logging crate (`log` + `env_logger`) or at
least decide on a convention.

### 6.7 Config is saved but `get_config_windows` is useless

**File:** `src/io/config.rs:78-83`

```rust
pub fn get_config_windows() -> Result<CliOptions, Box<dyn Error>> {
    Err(Box::new(io::Error::new(ErrorKind::Unsupported, "...")))
}
```

It always returns an error. On Windows, `snapshot()` in `chromium.rs` calls
this and immediately fails:

```rust
"windows" => get_config_windows()?,
```

So the program can't run on Windows at all. Either remove Windows support
entirely with a clear error at the CLI level, or implement it.

---

## 7. Summary: What to Fix First

| Priority | Fix | Why |
|---|---|---|
| **P0** | Remove `#![allow(unused)]` | Hides all other problems |
| **P0** | Fix ignored `Result` in `save_config_linux` (line 45) | Data loss bug |
| **P0** | Remove recursion in `request_path` | Stack overflow bug |
| **P1** | Replace `String` paths with `PathBuf`/`&Path` | Type safety |
| **P1** | Make error handling consistent (pick one strategy) | Maintainability |
| **P1** | Move `sqlite_parser` out of `chromium.rs` | Wrong file |
| **P1** | Fix `Road` `cli()` return type | Misleading API |
| **P2** | Fix `Browser::userdata_path` from HashMap to String | Over-engineered |
| **P2** | Remove dead code (`JSONBookmarkSchema`, `validate_schema`) | Cleanup |
| **P2** | Rename `schedular` → `scheduler` | Correctness |
| **P2** | Fix `generate_name` to not take full `Browser` | Overly coupled |
| **P2** | Extract generic utilities from `browsers.rs` | File placement |
| **P3** | Create custom error enum | Replace `Box<dyn Error>` |
| **P3** | Split CLI parsing from orchestration in `cli.rs` | Single responsibility |
| **P3** | Add tests | No coverage |
| **P3** | Remove unnecessary derives (Hash) | Cleanup |

---

## Quick Reference: Idiomatic Rust Patterns

### Paths: `PathBuf` for ownership, `&Path` for borrowing

```rust
// ❌ Bad
pub fn read_file(path: &str) -> ... { Path::new(path) }

// ✅ Good
pub fn read_file(path: &Path) -> ... { }

// ✅ Even better (accepts both &str and &Path)
pub fn read_file(path: impl AsRef<Path>) -> ... {
    let path = path.as_ref();
}
```

### Take `Copy` types by value

```rust
// ❌ Bad
fn foo(x: &SupportedOSs) {}

// ✅ Good (fieldless enums are Copy)
fn foo(x: SupportedOSs) {}
```

### Don't clone to read — borrow instead

```rust
// ❌ Bad
fn save(cli: &CliOptions) {
    let mut opts = cli.clone();
    // modify opts
}

// ✅ Good
fn save(cli: &CliOptions) {
    // work with &CliOptions directly
}
```

### Loop instead of recurse for user input

```rust
// ❌ Bad
fn ask() -> String {
    let input = get_input();
    if valid(&input) { input } else { ask() }
}

// ✅ Good
fn ask() -> String {
    loop {
        let input = get_input();
        if valid(&input) { return input; }
        eprintln!("Invalid, try again.");
    }
}
```

### Never ignore a `Result`

```rust
// ❌ Bad
do_something();  // returns Result<(), Error>
// ✅ Good
do_something()?;
// or
do_something().unwrap_or_else(|e| eprintln!("{e}"));
```
