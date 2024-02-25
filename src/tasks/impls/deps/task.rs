use crate::{
    config::JabuConfig,
    tools::JavaHome,
    tasks::{JabuTaskManager, JabuTask, TaskResult}
};

pub struct DepsTask;

impl JabuTask for DepsTask {
    fn description(&self) -> String {
        "Manages the project's dependencies.".to_string()
    }
    fn execute(&self, args: Vec<String>, jabu_config: &JabuConfig, java_home: &JavaHome) -> TaskResult {
        todo!()
    }
}
