use crate::{
    args::{options::Options, parser::ParsedArguments},
    tools::JavaHome,
};
use jabu_config::{
    fsutils::project_from_directory,
    model::{JabuProject, JABU_FILE_NAME},
};
use std::{collections::HashMap, path::PathBuf};

use super::{
    impls::{
        deps, BuildJabuTask, CleanTask, DisplayJabuTask, JPackageTask, JarTask, JavadocTask,
        PublishTask, Run, ScriptsTask,
    },
    TaskError, TaskResult,
};

/// Represents a task that it's supposed to be executed inside of a Jabu project.
pub trait JabuTask {
    /// Description of the task.
    fn description(&self) -> String;
    /// Executes the task with the given arguments and jabu config.
    fn execute(
        &self,
        args: Vec<String>,
        parsed_args: Option<ParsedArguments>,
        jabu_config: &JabuProject,
        java_home: &JavaHome,
    ) -> TaskResult;
    fn get_dependency_task_specs(&self) -> JabuTaskDependencySpec {
        JabuTaskDependencySpec::default()
    }

    /// Returns an slice of strings with the names
    /// of the required tools for the task (*i.e. `&["javac", "java"]`*)
    fn required_tools(&self) -> &[&'static str] {
        &[]
    }

    /// Returns the options specified by the task, if any.
    fn options(&self) -> Option<Options> {
        None
    }
}

pub struct JabuTaskDependencySpec {
    specs: HashMap<String, Vec<String>>,
}

impl Default for JabuTaskDependencySpec {
    fn default() -> Self {
        Self {
            specs: HashMap::new(),
        }
    }
}

impl JabuTaskDependencySpec {
    pub fn new(specs: HashMap<String, Vec<String>>) -> Self {
        Self { specs }
    }
}

/// Contains a collection [`JabuTask`] that can be executed.
pub struct JabuTaskManager {
    pub tasks: HashMap<String, Box<dyn JabuTask>>,
}

impl Default for JabuTaskManager {
    fn default() -> Self {
        Self {
            tasks: HashMap::new(),
        }
    }
}

impl JabuTaskManager {
    /// Creates the default jabu task manager for the top level tasks
    /// (*the main tasks available for jabu such as `new` or `version`.
    ///
    /// This is different from just using the [`Default::default()`] trait method,
    /// which would just return an empty jabu task manager.
    pub fn top_level_default() -> Self {
        let mut tasks: HashMap<String, Box<dyn JabuTask>> = HashMap::new();
        tasks.insert("run".to_string(), Box::new(Run::default()));
        tasks.insert("build".to_string(), Box::new(BuildJabuTask::default()));
        tasks.insert("info".to_string(), Box::new(DisplayJabuTask::default()));
        tasks.insert("scripts".to_string(), Box::new(ScriptsTask::default()));
        tasks.insert("clean".to_string(), Box::new(CleanTask::default()));
        tasks.insert("jar".to_string(), Box::new(JarTask::default()));
        tasks.insert("deps".to_string(), Box::new(deps::DepsSubtask::default()));
        tasks.insert("javadoc".to_string(), Box::new(JavadocTask::default()));
        tasks.insert("jpackage".to_string(), Box::new(JPackageTask::default()));
        tasks.insert("publish".to_string(), Box::new(PublishTask::default()));
        Self { tasks }
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
        self.tasks.iter().any(|(t_name, _)| task_name == t_name)
    }

    pub fn remove(&mut self, task_name: &str) -> Option<Box<dyn JabuTask>> {
        self.tasks.remove(task_name)
    }

    pub fn get_task(&self, task_name: &str) -> Option<&Box<dyn JabuTask>> {
        self.tasks.get(task_name)
    }

    pub fn execute(&self, task_name: &str, args: &Vec<String>, directory: &str) -> TaskResult {
        let task = if let Some(task) = self.get_task(task_name) {
            task
        } else {
            return Err(TaskError::NoSuchTask(task_name.to_string()));
        };

        //let jabu_config = JabuProject::try_from(PathBuf::from(directory).join(JABU_FILE_NAME))?;
        let jabu_project = project_from_directory(Some(PathBuf::from(directory)))?;
        let java_home = JavaHome::new()?;

        let required_tools = task.required_tools();
        let required_tools_status = java_home.check_required_tools(&required_tools.to_vec());

        // If any tool is missing
        if required_tools_status
            .iter()
            .any(|(_, available)| !available)
        {
            return Err(TaskError::MissingRequiredTaskTools(required_tools_status));
        }

        let parsed_args = if let Some(options) = task.options() {
            Some(ParsedArguments::new_with_options(args.clone(), &options)?)
        } else {
            None
        };

        for (task_name, task_args) in task.get_dependency_task_specs().specs {
            println!("=> Executing dependency task '{task_name}' with args '{task_args:?}'");

            if self.tasks.contains_key(&task_name) {
                self.execute(&task_name, args, directory)?
            } else {
                return Err(TaskError::DependencyTaskDoesntExist(task_name.clone()));
            }
        }

        task.execute(args.clone(), parsed_args, &jabu_project, &java_home)
    }
}
