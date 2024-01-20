use std::collections::{HashMap, HashSet};
use crate::args::parser::InvalidArgError;
use crate::tasks::impls::NewProjectTask;

pub type TaskResult = Result<(), TaskError>;

/// Represents the failure of a task.
#[derive(Debug)]
pub enum TaskError {
    /// Signifies the failure of the execution of an external command.
    CommandFailed(String, String),

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

    /// Generic error with a message attached to it.
    Generic(String),
}

impl std::fmt::Display for TaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Write specified messages.
        write!(f, "{:?}", self)
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

    fn contains_task_with_name(&self, task_name: &str) -> bool {
        self.tasks.iter()
            .any(|(t_name, _)| task_name == t_name)
    }

    pub fn remove(&mut self, task_name: &str) -> Option<Box<dyn Task>> {
        self.tasks.remove(task_name)
    }
}
