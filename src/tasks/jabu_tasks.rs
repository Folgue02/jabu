use crate::{config::{JabuConfig, JABU_FILE_NAME}, tools::JavaHome};
use std::{collections::HashMap, path::{Path, PathBuf}, error::Error};

use super::{TaskError, TaskResult, impls::{Run, DisplayJabuTask, BuildJabuTask}};

/// Represents a task that it's supposed to be executed inside of a Jabu project.
pub trait JabuTask {
    /// Description of the task.
	fn description(&self) -> String;
    /// Executes the task with the given arguments and jabu config.
	fn execute(&self, args: Vec<String>, jabu_config: &JabuConfig, java_home: &JavaHome) -> TaskResult;
}

pub struct JabuTaskManager {
	tasks: HashMap<String, Box<dyn JabuTask>>
}

impl Default for JabuTaskManager {
    fn default() -> Self {
        let mut tasks: HashMap<String, Box<dyn JabuTask>> = HashMap::new();
        tasks.insert("run".to_string(), Box::new(Run::default()));
        tasks.insert("build".to_string(), Box::new(BuildJabuTask::default()));
        tasks.insert("info".to_string(), Box::new(DisplayJabuTask::default()));
        Self {
            tasks
        }
    }
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

        let jabu_config = match JabuConfig::try_from(PathBuf::from(directory).join(JABU_FILE_NAME)) {
            Ok(cfg) => cfg,
            Err(e) => return Err(TaskError::IOError(e))
        };

        let java_home = match JavaHome::new() {
            Ok(java_home) => java_home,
            Err(e) => {
                eprintln!("Couldn't find a java installation path (it can be specified with the $JAVA_HOME variable)");
                return Err(TaskError::IOError(e))
            }
        };

        if !java_home.is_valid() {
            return Err(TaskError::Generic("The java environment found doesn't seem to be valid (doesn't contain all necessary tools)".to_string()));
        }

        println!("Java installation detected: {:?}", java_home.get_java_home());

    	task.execute(args, &jabu_config, &java_home)
    }
}
