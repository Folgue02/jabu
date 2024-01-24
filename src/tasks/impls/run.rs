use crate::tasks::{Task, JabuTask};
use crate::config::JabuConfig;
use crate::tools::JavaHome;

#[derive(Debug)]
pub struct Run {
}

impl Default for Run {
    fn default() -> Self {
        Self {}
    }
}

impl JabuTask for Run {
    fn description(&self) -> String {
        "Runs the current project".to_string()
    }

    fn execute(&self, args: Vec<String>, jabu_config: &JabuConfig, java_home: &JavaHome) -> crate::tasks::TaskResult {
        println!("Run task");
        Ok(())
    }
}
