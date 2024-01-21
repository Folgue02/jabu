use crate::tasks::JabuTask;

pub struct DisplayJabuTask {}

impl Default for DisplayJabuTask {
    fn default() -> Self {
        Self {}
    }
}

impl JabuTask for DisplayJabuTask {
    fn description(&self) -> String {
        "Displays the info of the current project.".to_string()
    }
    fn execute(&self, args: Vec<String>, jabu_config: &crate::config::JabuConfig) -> crate::tasks::TaskResult {
        println!("Project's configuration: {:?}", jabu_config);
        Ok(())
    }
}