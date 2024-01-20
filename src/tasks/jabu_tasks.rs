use crate::config::JabuConfig;
use std::{collections::HashMap, path::{Path, PathBuf}, error::Error};

use super::{TaskError, TaskResult};

/// Represents a task that it's supposed to be executed inside of a Jabu project.
pub trait JabuTask {
    /// Description of the task.
	fn description(&self) -> String;
    /// Executes the task with the given arguments and jabu config.
	fn execute(&self, args: Vec<String>, jabu_config: &JabuConfig) -> TaskResult;
}

pub struct JabuTaskManager {
	tasks: HashMap<String, Box<dyn JabuTask>>
}

impl JabuTaskManager {
    pub fn register_task(&mut self, task_name: &str, new_task: Box<dyn JabuTask>) -> bool {
        if self.contains_task_with_name(task_name) {
            false
        } else {
            self.tasks.insert(task_name.to_string(), new_task);
            true
        }
    }

    pub fn contains_task_with_name(&self, task_name: &str) -> bool {
        self.tasks.iter()
            .any(|(t_name, _)| task_name == t_name)
    }

    pub fn remove(&mut self, task_name: &str) -> Option<Box<dyn JabuTask>> {
        self.tasks.remove(task_name)
    }

    pub fn get_task(&self, task_name: &str) -> Option<&Box<dyn JabuTask>> {
    	self.tasks.get(task_name)
    }

    pub fn execute(&self, task_name: &str, args: Vec<String>, directory: &str) -> TaskResult {
    	let task = if let Some(task) = self.get_task(task_name) {
    		task
    	} else {
    		return Err(TaskError::NoSuchTask(task_name.to_string()));
    	};

        let jabu_config = match JabuConfig::try_from(PathBuf::from(directory)) {
            Ok(cfg) => cfg,
            Err(e) => return Err(TaskError::IOError(e))
        };

    	task.execute(args, &jabu_config)
    }
}
