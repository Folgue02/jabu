mod deps_task_manager;
mod list;

pub use deps_task_manager::*;
pub use list::*;
use crate::{tasks::{JabuTaskManager, JabuTask, TaskResult}, tools::JavaHome, config::{JabuConfig, JABU_FILE_NAME}};

#[derive(Default)]
pub struct DepsSubtask;

impl JabuTask for DepsSubtask {
    fn execute(&self, args: Vec<String>, jabu_config: &JabuConfig, java_home: &JavaHome) -> TaskResult {
        let task_manager = get_deps_task_manager();
        let task_name = if let Some(task_name) = args.get(0) {
            task_name.as_str()
        } else {
            "help"
        };

        task_manager.execute(task_name, args.clone(), ".")
    }

    fn description(&self) -> String {
        "Manage the project's dependencies.".to_string()
    }
}

