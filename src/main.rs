use std::process::exit;

use tasks::{TaskManager, JabuTaskManager, TaskError};

mod config;
mod tasks;
mod args;

#[cfg(test)]
mod tests;

fn main() {
    let mut args = std::env::args();
    let cwd = if let Ok(cwd) = std::env::current_dir() {
        cwd.to_string_lossy().to_string()
    } else {
        eprintln!("Couldn't get the current working directory.");
        exit(1)
    };
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

    match result {
        Ok(_) => println!("Done."),
        Err(e) => {
            eprintln!("Soemthing went wrong: {e}");
            exit(1)
        }
    }
    // let config = config::JabuConfig::default_of_name("test", config::java::ProjectType::Binary);
    // println!("{}", ron::ser::to_string_pretty(&config, ron::ser::PrettyConfig::default()).unwrap());
}
