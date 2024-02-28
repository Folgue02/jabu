use crate::{config::{JabuConfig, JABU_FILE_NAME}, tools::JavaHome};
use std::{
    collections::HashMap,
    path::PathBuf
};

use super::{
    TaskError, 
    TaskResult,
    impls::{
        deps, 
        Run,
        DisplayJabuTask,
        BuildJabuTask,
        ScriptsTask,
        CleanTask,
        JarTask,
    }
};

/// Represents a task that it's supposed to be executed inside of a Jabu project.
pub trait JabuTask {
    /// Description of the task.
	fn description(&self) -> String;
    /// Executes the task with the given arguments and jabu config.
	fn execute(&self, args: Vec<String>, jabu_config: &JabuConfig, java_home: &JavaHome) -> TaskResult;
    fn get_dependency_task_specs(&self) -> JabuTaskDependencySpec {
        JabuTaskDependencySpec::default()
    }

    /// Returns an slice of strings with the names
    /// of the required tools for the task (*i.e. `&["javac", "java"]`*)
    fn required_tools(&self) -> &[&'static str] {
        &[]
    }
}

pub struct JabuTaskDependencySpec {
    specs: HashMap<String, Vec<String>>
}

impl Default for JabuTaskDependencySpec {
    fn default() -> Self {
        Self {
            specs: HashMap::new()
        }
    }
}

impl JabuTaskDependencySpec {
    pub fn new(specs: HashMap<String, Vec<String>>) -> Self {
        Self {
            specs
        }
    }
}

/// Contains a collection [`JabuTask`] that can be executed.
pub struct JabuTaskManager {
    pub tasks: HashMap<String, Box<dyn JabuTask>>
}

impl Default for JabuTaskManager {
    fn default() -> Self {
        Self {
            tasks: HashMap::new()
        }
    }
}

impl JabuTaskManager {
    /// Creates the default jabu task manager for the top level tasks 
    /// (*the main tasks available for jabu such as `new` or `version`.
    pub fn top_level_default() -> Self {
        let mut tasks: HashMap<String, Box<dyn JabuTask>> = HashMap::new();
        tasks.insert("run".to_string(), Box::new(Run::default()));
        tasks.insert("build".to_string(), Box::new(BuildJabuTask::default()));
        tasks.insert("info".to_string(), Box::new(DisplayJabuTask::default()));
        tasks.insert("scripts".to_string(), Box::new(ScriptsTask::default()));
        tasks.insert("clean".to_string(), Box::new(CleanTask::default()));
        tasks.insert("jar".to_string(), Box::new(JarTask::default()));
        tasks.insert("deps".to_string(), Box::new(deps::DepsSubtask::default()));
        Self {
            tasks
        }
    }

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
            Err(e) => return Err(TaskError::InvalidConfig(Box::new(e)))
        };

        let java_home = match JavaHome::new() {
            Ok(java_home) => java_home,
            Err(_) => {
                eprintln!("Couldn't find a java installation path (it can be specified with the $JAVA_HOME variable)");
                return Err(TaskError::MissingJavaEnvironment);
            }
        };

        
        /* TODO: Remove me
        if !java_home.is_valid() {
            return Err(
                TaskError::InvalidJavaEnvironment(java_home.get_java_home().to_string_lossy().to_string())
            )
        }
        */

        for (task_name, task_args) in task.get_dependency_task_specs().specs {
            println!("=> Executing dependency task '{task_name}' with args '{task_args:?}'");
            if let Some(dep_task) = self.get_task(&task_name) {
                match dep_task.execute(task_args, &jabu_config, &java_home) {
                    Ok(_) => (),
                    Err(e) => {
                        return Err(TaskError::DependencyTaskFailed { task_name: task_name.clone(), description: e.to_string() })
                    }
                }
            } else {
                return Err(TaskError::DependencyTaskDoesntExist(task_name.clone()));
            }
        }

    	task.execute(args, &jabu_config, &java_home)
    }
}
