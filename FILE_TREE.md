bookmark-tree/
├── Cargo.toml              # Rust project manifest. Dependencies: clap, serde, serde_json,
│                           #   dialoguer, colored, glob, schemars, systemd.
├── Cargo.lock              # Auto-generated dependency lockfile.
├── README.md               # Brief description: browser bookmark snapshot utility.
├── todo.md                 # Dev roadmap: completed items, remaining work (Gecko support,
│                           #   Windows support, docs, tests, interactive UI, etc.).
├── .gitignore              # Ignores /target build directory.
│
├── assets/
│   ├── bookmark-tree.service   # systemd service unit template (injected at compile time
│   │                           #   via include_str! into schedular/linux.rs). Defines
│   │                           #   ExecStart with a dev binary path.
│   └── pj-details.png          # Project screenshot/diagram.
│
├── src/
│   ├── main.rs                 # Entry point. Declares 5 modules; calls cli::cli().
│   │
│   ├── cli.rs                  # CLI argument parsing (clap builder API). Supports:
│   │                           #   --browser, --outputpath, --routine, --count.
│   │                           #   Orchestrates the full flow: OS detection → browser
│   │                           #   discovery → config save → snapshot → schedule.
│   │
│   ├── types.rs                # Core data types:
│   │                           #   - SupportedBrowsers (Brave/Chrome/Firefox) with
│   │                           #     default_path() per OS
│   │                           #   - Browser struct (name, paths, store_type)
│   │                           #   - BookmarkStoreType (JSON/SQLite)
│   │                           #   - SupportedOSs (Windows/Linux)
│   │                           #   - Routine (Day/Week/Month)
│   │                           #   - CliOptions (aggregates all CLI state)
│   │
│   ├── io/
│   │   ├── mod.rs              # Module declarations: browsers, config.
│   │   ├── browsers.rs         # Browser bookmark file discovery via glob patterns.
│   │   │                       #   search_browsers(): iterates browsers, finds bookmark
│   │   │                       #     files on disk using pattern_builder().
│   │   │                       #   request_path(): interactive fallback if no bookmarks
│   │   │                       #     found automatically (dialoguer prompts).
│   │   │                       #   check_path(), get_home_directory(), get_input().
│   │   │
│   │   └── config.rs           # Config persistence (CLI options as JSON).
│   │                           #   save_config_linux(): writes to
│   │                           #     ~/.config/BookmarkSnapshot/options_config.json.
│   │                           #   get_config_linux(): reads + deserializes config back.
│   │                           #   Windows stubs (save/return Unsupported error).
│   │                           #   validate_schema() placeholder.
│   │
│   └── parser/
│       ├── mod.rs              # Module declarations + design notes (Gecko SQLite copy-
│       │                       #   before-read tip, Chromium JSON format).
│       ├── types.rs            # Serde structs mirroring Chromium bookmark JSON:
│       │                       #   ChromiumBookmarks → BookmarkRoots (bookmark_bar,
│       │                       #   other, synced) → recursive BookmarkNode (id/name/
│       │                       #   type/url/children), NodeType (Url/Folder).
│       │
│       ├── chromium.rs         # Chromium bookmark snapshotter. snapshot() reads config,
│       │                       #   dispatches json_parser() or sqlite_parser() per
│       │                       #   browser store_type. json_parser() deserializes the
│       │                       #   JSON bookmark file and writes a timestamped copy to
│       │                       #   ~/Documents/Bookmarks Snapshots/<browser>/.
│       │                       #   sqlite_parser() is a no-op stub.
│       │
│       └── gecko.rs            # Firefox/Gecko parser stub: empty gecko_parser() fn.
│                               #   Not yet implemented (todo.md).
│
└── schedular/
    ├── mod.rs                  # Module declarations: linux.
    ├── linux.rs                # systemd user timer/service scheduler. Reads config,
    │                           #   renders timer unit (OnBootSec=5min, OnUnitActiveSec
    │                           #   based on Routine + count), writes .service and .timer
    │                           #   files to ~/.config/systemd/user/.
    │
    └── sketches.md             # Design research notes: comparison of systemd timers vs
                                #   cron, two-part unit architecture, user vs system
                                #   systemd instances, Persistent flag, unit syntax
                                #   prototypes.
