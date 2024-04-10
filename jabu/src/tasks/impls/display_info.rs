use crate::{tasks::JabuTask, tools::JavaHome, args::parser::ParsedArguments};
use jabu_config::model::JabuProject;

#[derive(Default)]
pub struct DisplayJabuTask {}

impl JabuTask for DisplayJabuTask {
    fn description(&self) -> String {
        "Displays the info of the current project.".to_string()
    }
    fn execute(&self, args: Vec<String>, _: Option<ParsedArguments>, jabu_config: &JabuProject, java_home: &JavaHome) -> crate::tasks::TaskResult {
        println!("Project's configuration: {:?}", jabu_config);
        Ok(())
    }
}
