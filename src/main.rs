use std::process::exit;

use tasks::{TaskManager, JabuTaskManager, TaskError};
use chrono;

mod config;
mod tasks;
mod args;
mod tools;
mod utils;

#[cfg(test)]
mod tests;

pub const VERSION: &'static str = "0.0.2";

fn main() {
    let mut args = std::env::args();
    let cwd = std::env::current_dir().expect("Couldn't get the current working directory")
        .to_string_lossy()
        .to_string();

    args.next();
    let result = match args.next() {
        Some(task_name) => {
            let task_manager = TaskManager::default();
            let jabu_task_manager = JabuTaskManager::default();

            if task_manager.contains_task_with_name(task_name.as_str()) {
                task_manager.execute(task_name.as_str(), args.collect(), &cwd)
            } else if jabu_task_manager.contains_task_with_name(task_name.as_str()) {
                jabu_task_manager.execute(task_name.as_str(), args.collect(), &cwd)
            } else {
                Err(TaskError::NoSuchTask(task_name))
            }
        }
        None => {
            eprintln!("You didn't specify a task to be run!");
            exit(1)
        }
    };

    let _end_timestamp = chrono::offset::Local::now();
    match result {
        Ok(_) => {
            // TODO: Display OK message.
        }
        Err(e) => {
            eprintln!("Something went wrong: {e}");
            exit(1)
        }
    }
}
