use std::{
    error::Error,
    fs::OpenOptions,
    io::{BufWriter, ErrorKind::NotFound, Write},
};

use crate::{
    io::{
        self,
        browsers::get_home_directory,
        config::{self, get_config_linux},
    },
    types::Routine,
};

// we use monotonic timers
const SERVICE_UNIT: &str = include_str!("../../assets/bookmark-tree.service");
const SYSTEMD_USER_DIRECTORY: &str = ".config/systemd/user/";
fn render_timer_unit() -> Result<(String), Box<dyn Error>> {
    let config = get_config_linux()?;
    let routine = if let Some(routine) = config.routine {
        routine.clone()
    } else {
        // Definitely the error doesn't match our case
        return Err(Box::new(std::io::Error::new(NotFound, "no routine found")));
    };
    let count = config.routine_count;
    let on_unit_active_sec = match routine {
        Routine::Day => {
            format!("{}d", count)
        }
        Routine::Week => {
            format!("{}w", count)
        }
        Routine::Month => {
            format!("{}d", count * 30)
        }
    };
    let timer_unit = format!(
        "[Unit]\n\
         Description=Timer for bookmark tree\n\n\
         [Timer]\n\
         OnBootSec=5min\n\
         OnUnitActiveSec={}\n\
         Persistent=true\n\n\
         [Install]\n\
         WantedBy=timers.target\n",
        on_unit_active_sec
    );

    // now we have both timer and services units, we can inject them into user systemd directory
    Ok(timer_unit)
}
pub fn schedule() -> std::io::Result<()> {
    let service_unit = SERVICE_UNIT;
    let timer_unit = if let Ok(unit) = render_timer_unit() {
        unit
    } else {
        // this is not ideal at all, we eliminated the error come from render fucntio
        return Err(std::io::Error::new(NotFound, "not routine found"));
    };
    let service_unit_path = format!(
        "{}/{}/bookmark-tree.service",
        get_home_directory(),
        SYSTEMD_USER_DIRECTORY
    );
    let timer_unit_path = format!(
        "{}/{}/bookmark-tree.timer",
        get_home_directory(),
        SYSTEMD_USER_DIRECTORY
    );
    let servie_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(service_unit_path)?;
    let timer_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(timer_unit_path)?;

    let mut service_writer = BufWriter::new(servie_file);
    let mut timer_writer = BufWriter::new(timer_file);

    write!(timer_writer, "{}", timer_unit)?;
    write!(service_writer, "{}", service_unit)?;
    
    println!("snapshot routine scheduled successfully");
    Ok(())
}
