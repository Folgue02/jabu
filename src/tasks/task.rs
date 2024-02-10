use std::collections::{HashMap, HashSet};
use crate::args::parser::InvalidArgError;
use crate::tasks::impls::{VersionTask, NewProjectTask};

pub type TaskResult = Result<(), TaskError>;

/// Represents the failure of a task.
#[derive(Debug)]
pub enum TaskError {
    /// Signifies the failure of the execution of an external command.
    CommandFailed{command: String, description: String},

    /// Invalid configuration, this can mean that the configuration of the
    /// projet was wrong.
    InvalidConfig(Option<String>),

    /// The provided task didn't exist.
    NoSuchTask(String),

    /// An `IO` error has occurred. This variant contains
    /// a `std::io::Error` representing the cause of the error.
    IOError(std::io::Error),

    /// Invalid arguments have been supplied.
    InvalidArguments(HashSet<InvalidArgError>),

    /// A task has specified a non existent task as 
    /// dependency.
    DependencyTaskDoesntExist(String),

    /// A dependency task specified by another task has failed.
    DependencyTaskFailed{ task_name: String, description: String},

    /// Generic error with a message attached to it.
    Generic(String),
}

impl std::fmt::Display for TaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::Generic(desc) => format!("Something went wrong: {desc}"),
            Self::IOError(io_error) => format!("An IO error has occurred: {io_error}"),
            Self::DependencyTaskFailed { task_name, description } => {
                format!("While executing a task there was an error executing its dependency task '{task_name}': {description}")
            }
            Self::DependencyTaskDoesntExist(dependency_task) => {
                format!("A task called a dependency task '{dependency_task}' which doesn't exist.")
            }
            Self::InvalidConfig(possible_description) => {
                if let Some(description) = possible_description {
                    format!("The project's jabu configuration is invalid: {description}")
                } else {
                    format!("The project's jabu configuration is invalid.")
                }
            }
            Self::NoSuchTask(task_name) => format!("Task with name '{task_name}' doesn't exist."),
            Self::CommandFailed {command, description} => {
                format!("Command '{command}' due to the following error/error code: {description}")
            }
            Self::InvalidArguments(errors) => {
                let error_list_compiled = errors.iter()
                    .enumerate()
                    .map(|(index, err)| {
                        format!("{} : {err}", index + 1)
                    }).collect::<Vec<String>>()
                .join("\n");
                format!("Invalid arguments:\n{error_list_compiled}")
            }
        };
        // TODO: Write specified messages.
        write!(f, "{msg}")
    }
}

impl std::error::Error for TaskError {}

/// Represents a generic task that may or may not be executed inside of a
/// jabu project.
pub trait Task: std::fmt::Debug {
    /// Description of the task
    fn description(&self) -> String;

    /// Executes the task with the given arguments.
    fn execute(&self, args: Vec<String>) -> TaskResult;
}

#[derive(Debug)]
pub struct TaskManager {
    tasks: HashMap<String, Box<dyn Task>>
}

impl Default for TaskManager {
    fn default() -> Self {
        let mut tasks: HashMap<String, Box<dyn Task>> = HashMap::new();
        tasks.insert("new".to_string(), Box::new(NewProjectTask {}));
        tasks.insert("version".to_string(), Box::new(VersionTask::default()));
        Self {
            tasks
        }
    }
}

impl TaskManager {
    pub fn register_task(&mut self, task_name: &str, new_task: Box<dyn Task>) -> bool {
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

    pub fn remove(&mut self, task_name: &str) -> Option<Box<dyn Task>> {
        self.tasks.remove(task_name)
    }

    pub fn get_task(&self, task_name: &str) -> Option<&Box<dyn Task>> {
        self.tasks.get(task_name)
    }

    pub fn execute(&self, task_name: &str, args: Vec<String>, directory: &str) -> TaskResult {
        let task = if let Some(task) = self.get_task(task_name) {
            task
        } else {
            return Err(TaskError::NoSuchTask(task_name.to_string()));
        };

        task.execute(args)
    }
}
