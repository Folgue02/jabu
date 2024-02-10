use std::process::exit;

use tasks::{TaskManager, JabuTaskManager, TaskError};
use chrono;

mod config;
mod tasks;
mod args;
mod tools;
mod utils;
mod dslapi;

#[cfg(test)]
mod tests;

pub const VERSION: &'static str = "0.0.3";

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

    let task_manager = TaskManager::default();
    let jabu_task_manager = JabuTaskManager::default();

    // TODO: separate this into a different reusable
    // function.
    let result = if task_manager.contains_task_with_name(&task_name) {
        task_manager.execute(task_name.as_str(), args.collect(), &cwd)
    } else if jabu_task_manager.contains_task_with_name(task_name.as_str()) {
        jabu_task_manager.execute(task_name.as_str(), args.collect(), &cwd)
    } else {
        Err(TaskError::NoSuchTask(task_name.clone()))
    };

    let _end_timestamp = chrono::offset::Local::now();
    match result {
        Ok(_) => {
            // TODO: Ok message
        }
        Err(e) => {
            eprintln!("Failure:\n{e}");
            exit(1)
        }
    }
}
