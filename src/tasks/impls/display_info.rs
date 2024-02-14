use crate::{tasks::JabuTask, tools::JavaHome};

#[derive(Default)]
pub struct DisplayJabuTask {}

impl JabuTask for DisplayJabuTask {
    fn description(&self) -> String {
        "Displays the info of the current project.".to_string()
    }
    fn execute(&self, args: Vec<String>, jabu_config: &crate::config::JabuConfig, java_home: &JavaHome) -> crate::tasks::TaskResult {
        println!("Project's configuration: {:?}", jabu_config);
        Ok(())
    }
}
