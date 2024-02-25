use std::process::exit;

use tasks::{TaskError, GeneralTaskManager};
use chrono;

use crate::tools::JavaHome;

mod config;
mod tasks;
mod args;
mod tools;
mod utils;
mod dslapi;

#[cfg(test)]
mod tests;

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

pub const VERSION: &'static str = built_info::PKG_VERSION;

fn main() {
    let mut args = std::env::args();
    let cwd = std::env::current_dir().expect("Couldn't get the current working directory")
        .to_string_lossy()
        .to_string();

    args.next();
    let task_name = match args.next() {
        Some(task_name) => task_name,
        None => {
            eprintln!("No task specified!");
            exit(1);
        }
    };

    let general_task_manager = GeneralTaskManager::new(
        tasks::JabuTaskManager::top_level_default(),
        tasks::TaskManager::top_level_default()
    );

    let result = general_task_manager.execute(&task_name, args.collect(), &cwd);
    let _end_timestamp = chrono::offset::Local::now();
    match result {
        Err(e) => {
            handle_error(e);
        }
        _ => ()
    }
}

fn handle_error(e: TaskError) -> ! {
    eprintln!("Failure:\n{e}");

    // TODO: Remove or keep?
    if let TaskError::InvalidJavaEnvironment(_) = e {
        eprintln!("JDK tools status: ");
        let java_home = JavaHome::new().unwrap();
        java_home.get_tools()
            .iter()
            .for_each(|(tool_name, tool_path)| {
                eprintln!("{tool_name}: {tool_path:?}");
            });
    }
    exit(1)
}
