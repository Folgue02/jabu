use crate::args::{
    options::Options,
    parser::{InvalidArgError, ParsedArguments},
};
use crate::tasks::{
    impls::{HealthCheckTask, NewProjectTask, VersionTask},
    JabuTask, JabuTaskManager,
};
use jaburepo::error::RepositoryOperationError;
use prettytable::{color, Attr, Cell, Row};
use std::collections::{HashMap, HashSet};
use jabu_config::fsutils::ProjectLoadingError;

pub type TaskResult = Result<(), TaskError>;

/// Represents the failure of a task.
#[derive(Debug)]
pub enum TaskError {
    /// Couldn't find any java environment.
    MissingJavaEnvironment,

    /// The java environment that has been found is not valid. (*it may be missing
    /// some of the utilities*). The string of the variant represents the path to the
    /// java home/environment.
    ///
    /// # See
    /// - [`TaskError::MissingRequiredTaskTools`]
    InvalidJavaEnvironment(String),

    /// Represents the lack of tools required for a task. (*i.e. the 'run' task
    /// requires the 'javac' and 'java' tools to be available, if any of them aren't,
    /// this variant will hold a map with the required tools and their status.
    ///
    /// # Example
    /// - 'java'  => `true` (because it is available),
    /// - 'javac' => `false` (because it's not not available)
    ///
    /// # See
    /// - [`crate::tools::JavaHome::check_required_tools`]
    MissingRequiredTaskTools(HashMap<&'static str, bool>),

    /// Signifies the failure of the execution of an external command.
    CommandFailed {
        command: String,
        description: String,
    },

    /// Invalid configuration, this can mean that the configuration of the
    /// projet was wrong.
    InvalidConfig(Box<dyn std::error::Error>),

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

    /// A resource is unavailable, this can represent a dependency
    /// that cannot be found or downloaded.
    UnavailableResource {
        /// The name of the resource (*this can be the artifact's
        /// name*)
        resource_name: String,

        /// Error message explaining what happened.
        error: Option<String>,
    },

    /// Generic error with a message attached to it.
    Generic(String),
}

impl From<std::io::Error> for TaskError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

impl From<reqwest::Error> for TaskError {
    fn from(value: reqwest::Error) -> Self {
        let resource_name = if let Some(url) = value.url() {
            url.as_str()
        } else {
            "No resource name given."
        };
        Self::UnavailableResource {
            resource_name: resource_name.to_string(),
            error: Some(format!("Status code of the response: {:?}", value.status())),
        }
    }
}

impl From<HashSet<InvalidArgError>> for TaskError {
    fn from(value: HashSet<InvalidArgError>) -> Self {
        Self::InvalidArguments(value)
    }
}

impl From<jaburepo::error::RepositoryOperationError> for TaskError {
    fn from(value: jaburepo::error::RepositoryOperationError) -> Self {
        match value {
            RepositoryOperationError::IoError(e) => e.into(),
            RepositoryOperationError::ArtifactNotFound(e) => TaskError::UnavailableResource {
                resource_name: e.to_string(),
                error: None
            }
        }
    }
}

impl From<ProjectLoadingError> for TaskError {
    fn from(value: jabu_config::fsutils::ProjectLoadingError) -> Self {
        match value {
            ProjectLoadingError::IoError(e) => Self::IOError(e),
            ProjectLoadingError::FileParsingError(e) => Self::InvalidConfig(Box::new(e))
        }
    }
}

impl std::fmt::Display for TaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::MissingJavaEnvironment => "No java environment could be found. (you can specify one with the environment variable 'JAVA_HOME'".to_string(),
            Self::InvalidJavaEnvironment(env) => format!("'{env}' as a java home is invalid (it might be missing some tools)"),
            Self::MissingRequiredTaskTools(tool_map) => {
                // TODO: Add colors / Put it in a table.
                let body = tool_map.iter()
                    .map(|(tool_name, availability)| format!("   {} : {}", tool_name, availability))
                    .collect::<Vec<String>>()
                    .join("\n");
                format!("Missing required tools for the given task:\n{body}")
            }
            Self::Generic(desc) => format!("Something went wrong: {desc}"),
            Self::IOError(io_error) => format!("An IO error has occurred: {io_error}"),
            Self::DependencyTaskFailed { task_name, description } => {
                format!("While executing a task there was an error executing its dependency task '{task_name}': {description}")
            }
            Self::DependencyTaskDoesntExist(dependency_task) => {
                format!("A task called a dependency task '{dependency_task}' which doesn't exist.")
            }
            Self::InvalidConfig(description) => {
                format!("The project's jabu configuration is invalid: {description}")
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
            Self::UnavailableResource { resource_name, error } => {
                if let Some(error) = error {
                    format!("The following resource is unavailable: {resource_name}\n\tCause: {error}")
                } else {
                    format!("The following resource is unavailable: {resource_name}")
                }
            }
        };
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
    fn execute(&self, args: Vec<String>, parsed_options: Option<ParsedArguments>) -> TaskResult;

    /// Returns the options used by the task. If the task doesn't use
    /// options, this method should return `None`.
    ///
    /// # See
    /// * [`crate::args::options::Options`]
    ///
    /// ***NOTE***: By default, this trait method will return `None`.
    fn options(&self) -> Option<Options> {
        None
    }
}

#[derive(Debug)]
pub struct TaskManager {
    tasks: HashMap<String, Box<dyn Task>>,
}

impl Default for TaskManager {
    fn default() -> Self {
        Self {
            tasks: HashMap::new(),
        }
    }
}

impl TaskManager {
    /// Creates the default task manager for the top level tasks
    /// (*the main tasks available for jabu such as `new` or `version`*).
    ///
    /// This is different from just using the [`Default::default()`] trait method,
    /// which would just return an empty task manager.
    pub fn top_level_default() -> Self {
        let mut tasks: HashMap<String, Box<dyn Task>> = HashMap::new();
        tasks.insert("new".to_string(), Box::new(NewProjectTask {}));
        tasks.insert("version".to_string(), Box::new(VersionTask::default()));
        tasks.insert("health".to_string(), Box::new(HealthCheckTask::default()));
        Self { tasks }
    }

    /// Registers a task in the task manager. If there already is task with the given name,
    /// this method will return `false`, if the task is added otherwise, `true`.
    pub fn register_task(&mut self, task_name: &str, new_task: Box<dyn Task>) -> bool {
        if self.get_task(task_name).is_some() {
            false
        } else {
            self.tasks.insert(task_name.to_string(), new_task);
            true
        }
    }

    /// Removes a task with the given name and returns it. If there was no
    /// task with such name, `None` is returned.
    pub fn remove(&mut self, task_name: &str) -> Option<Box<dyn Task>> {
        self.tasks.remove(task_name)
    }

    /// Returns a task with the given name, if the task doesn't exist,
    /// `None` will be returned.
    pub fn get_task(&self, task_name: &str) -> Option<&Box<dyn Task>> {
        self.tasks.get(task_name)
    }

    /// Executes the task with the given name, and passing its args. If there is no task
    /// with the given name, this method will return [`TaskError::NoSuchTask`].
    /// Any other variant of [`TaskError`] is provided by the execution of the task
    /// itself.
    pub fn execute(&self, task_name: &str, args: Vec<String>, directory: &str) -> TaskResult {
        let task = if let Some(task) = self.get_task(task_name) {
            task
        } else {
            return Err(TaskError::NoSuchTask(task_name.to_string()));
        };

        let parsed_args = if let Some(options) = task.options() {
            Some(ParsedArguments::new_with_options(args.clone(), &options)?)
        } else {
            None
        };

        task.execute(args, parsed_args)
    }
}

/// The `GeneralTaskManager` is a task manager with the same purposes
/// as the normal `TaskManager` and the `JabuTaskManager`, but has the
/// additional purpose of containing those task managers together and
/// centralizing all the use logic.
pub struct GeneralTaskManager {
    jabu_task_manager: JabuTaskManager,
    task_manager: TaskManager,
}

impl Default for GeneralTaskManager {
    fn default() -> Self {
        Self {
            jabu_task_manager: JabuTaskManager::default(),
            task_manager: TaskManager::default(),
        }
    }
}

impl GeneralTaskManager {
    pub fn new(jabu_task_manager: JabuTaskManager, task_manager: TaskManager) -> Self {
        Self {
            jabu_task_manager,
            task_manager,
        }
    }

    /// Checks if any of the task managers contain a task with the given
    /// name.
    pub fn contains_task_with_name(&self, task_name: &str) -> bool {
        self.jabu_task_manager.contains_task_with_name(task_name)
            || self.task_manager.get_task(task_name).is_some()
            || task_name == "help"
    }

    /// Executes the task associated with the given task name. This task may be in any of the
    /// stored task managers. This method may return `TaskError:NoSuchTask` if there is no
    /// task with the given name in any of the stored task managers.
    pub fn execute(&self, task_name: &str, args: Vec<String>, directory: &str) -> TaskResult {
        // TODO: Refactor :D
        if task_name == "help" {
            self.list_tasks();
            return Ok(());
        }

        if let Some(_) = self.jabu_task_manager.get_task(task_name) {
            self.jabu_task_manager.execute(task_name, &args, directory)
        } else if let Some(_) = self.task_manager.get_task(task_name) {
            self.task_manager.execute(task_name, args, directory)
        } else {
            Err(TaskError::NoSuchTask(task_name.to_string()))
        }
    }

    fn list_tasks(&self) {
        let mut table = prettytable::Table::new();
        table.set_format(*prettytable::format::consts::FORMAT_BORDERS_ONLY);
        table.add_row(Row::new(vec![Cell::new("TASKS")
            .with_style(Attr::Bold)
            .with_style(Attr::BackgroundColor(color::BRIGHT_BLACK))
            .with_style(Attr::ForegroundColor(color::WHITE))]));
        table.set_titles(Row::new(vec![
            Cell::new("Task name"),
            Cell::new("Description"),
        ]));
        self.task_manager.tasks.iter().for_each(|(name, task)| {
            table.add_row(Row::new(vec![
                Cell::new(&name)
                    .with_style(Attr::Bold)
                    .with_style(Attr::ForegroundColor(color::BLUE)),
                Cell::new(&task.description()),
            ]));
        });
        table.add_empty_row();
        table.add_row(Row::new(vec![Cell::new("JABU TASKS")
            .with_style(Attr::Bold)
            .with_style(Attr::BackgroundColor(color::BRIGHT_BLACK))
            .with_style(Attr::ForegroundColor(color::WHITE))]));
        self.jabu_task_manager
            .tasks
            .iter()
            .for_each(|(name, task)| {
                table.add_row(Row::new(vec![
                    Cell::new(&name)
                        .with_style(Attr::Bold)
                        .with_style(Attr::ForegroundColor(color::BLUE)),
                    Cell::new(&task.description()),
                ]));
            });
        table.printstd();
    }

    /// Registers a new [`crate::tasks::Task`] in the internal [`crate::tasks::TaskManager`]
    /// of the manager.
    pub fn register_task(&mut self, task_name: &str, task: Box<dyn Task>) -> bool {
        self.task_manager.register_task(task_name, task)
    }

    /// Registers a new [`crate::tasks::Task`] in the internal [`crate::tasks::JabuTaskManager`]
    /// of the manager.
    pub fn register_jabu_task(&mut self, task_name: &str, jabu_task: Box<dyn JabuTask>) -> bool {
        self.jabu_task_manager.register_task(task_name, jabu_task)
    }
}
