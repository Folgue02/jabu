use crate::tasks::{Task, JabuTask};
use crate::config::JabuConfig;

#[derive(Debug)]
pub struct Run {
}

impl JabuTask for Run {
    fn description(&self) -> String {
        "Runs the current project".to_string()
    }

    fn execute(&self, args: Vec<String>, jabu_config: &JabuConfig) -> crate::tasks::TaskResult {
        println!("Run task");
        Ok(())
    }
}
