pub type TaskResult = Result<(), TaskError>;

/// Represents the failure of a task.
#[derive(Debug)]
pub enum TaskError {
    CommandFailed(String, String),
    InvalidConfig(Option<String>),
    Generic(String),
}

pub trait Task {
    fn description(&self) -> String;
    fn execute(&self, args: Vec<String>) -> TaskResult;
}

pub struct TaskManager {
    tasks: Vec<Box<dyn Task>>
}

impl Default for TaskManager {
    fn default() -> Self {
        Self {
            tasks: Vec::new()
        }
    }
}
