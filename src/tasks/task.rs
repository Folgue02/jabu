use crate::args::parser::InvalidArgError;
use prettytable::{Table, Row, Attr, Cell, color, format::FormatBuilder};
use crate::{tasks::{impls::{NewProjectTask, VersionTask}, JabuTaskManager}, tools::JavaHome, config::{JabuConfig, JABU_FILE_NAME}};
use std::{io::Write, collections::{HashMap, HashSet}, path::PathBuf};

pub type TaskResult = Result<(), TaskError>;

/// Represents the failure of a task.
#[derive(Debug)]
pub enum TaskError {
    /// Couldn't find any java environment.
    MissingJavaEnvironment,

    /// The java environment that has been found is not valid. (*it may be missing
    /// some of the utilities*)
    InvalidJavaEnvironment(String),

    /// Signifies the failure of the execution of an external command.
    CommandFailed {
        command: String,
        description: String,
    },

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
    DependencyTaskFailed {
        task_name: String,
        description: String,
    },

    /// Generic error with a message attached to it.
    Generic(String),
}

impl From<std::io::Error> for TaskError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

impl std::fmt::Display for TaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::MissingJavaEnvironment => "No java environment could be found. (you can specify one with the environment variable 'JAVA_HOME'".to_string(),
            Self::InvalidJavaEnvironment(env) => format!("'{env}' as a java home is invalid (it might be missing some tools)"),
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
                format!("Command '{command}' with the following error/error code: {description}")
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
    tasks: HashMap<String, Box<dyn Task>>,
}

impl Default for TaskManager {
    fn default() -> Self {
        let mut tasks: HashMap<String, Box<dyn Task>> = HashMap::new();
        tasks.insert("new".to_string(), Box::new(NewProjectTask {}));
        tasks.insert("version".to_string(), Box::new(VersionTask::default()));
        Self { tasks }
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
        self.tasks.iter().any(|(t_name, _)| task_name == t_name)
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

/// The `GeneralTaskManager` is a task manager with the same purposes 
/// as the normal `TaskManager` and the `JabuTaskManager`, but has the 
/// additional purpose of containing those task managers together and 
/// centralizing all the use logic.
pub struct GeneralTaskManager {
    jabu_task_manager: JabuTaskManager,
    task_manager: TaskManager
}

impl Default for GeneralTaskManager {
    fn default() -> Self {
        Self {
            jabu_task_manager: JabuTaskManager::default(),
            task_manager: TaskManager::default()
        }
    }
}

impl GeneralTaskManager {
    pub fn new(jabu_task_manager: JabuTaskManager, task_manager: TaskManager) -> Self {
        Self {
            jabu_task_manager,
            task_manager
        }
    }

    /// Checks if any of the task managers contain a task with the given
    /// name.
    pub fn contains_task_with_name(&self, task_name: &str) -> bool {
        self.jabu_task_manager.contains_task_with_name(task_name)
            || self.task_manager.contains_task_with_name(task_name)
            || task_name == "help"
    }

    /// Executes the task associated with the given task name. This task may be in any of the
    /// stored task managers. This method may return `TaskError:NoSuchTask` if there is no
    /// task with the given name in any of the stored task managers.
    pub fn execute(&self, task_name: &str, args: Vec<String>, directory: &str) -> TaskResult {
        // TODO: Refactor :D
        if task_name == "help" {
            self.list_tasks();
            return Ok(())
       }

        if let Some(_) = self.jabu_task_manager.get_task(task_name) {
            self.jabu_task_manager.execute(task_name, args, directory)
        } else if let Some(_) = self.task_manager.get_task(task_name) {
            self.task_manager.execute(task_name, args, directory)
        } else {
            Err(TaskError::NoSuchTask(task_name.to_string()))
        }
    }

    fn list_tasks(&self) {
        // TODO: Change
        let mut table = prettytable::Table::new();
        table.set_format(*prettytable::format::consts::FORMAT_BORDERS_ONLY);
        table.add_row(
            Row::new(
                vec![
                    Cell::new("TASKS")
                        .with_style(Attr::Bold)
                        .with_style(Attr::BackgroundColor(color::BRIGHT_BLACK))
                        .with_style(Attr::ForegroundColor(color::WHITE))
                ]
            )
        );
        table.set_titles(
            Row::new(vec![
                Cell::new("Task name"),
                Cell::new("Description")
            ])
        );
        self.task_manager.tasks.iter()
            .for_each(|(name, task)| {
                table.add_row(
                    Row::new(vec![
                        Cell::new(&name)
                            .with_style(Attr::Bold)
                            .with_style(Attr::ForegroundColor(color::BLUE)),
                        Cell::new(&task.description())
                    ])
                );
            });
        table.add_empty_row();
        table.add_row(
            Row::new(
                vec![
                    Cell::new("JABU TASKS")
                        .with_style(Attr::Bold)
                        .with_style(Attr::BackgroundColor(color::BRIGHT_BLACK))
                        .with_style(Attr::ForegroundColor(color::WHITE))
                ]
            )
        );
        self.jabu_task_manager.tasks.iter()
            .for_each(|(name, task)| {
                table.add_row(
                    Row::new(vec![
                        Cell::new(&name)
                            .with_style(Attr::Bold)
                            .with_style(Attr::ForegroundColor(color::BLUE)),
                        Cell::new(&task.description())
                    ])
                );
            });
        table.printstd();
    }
}
