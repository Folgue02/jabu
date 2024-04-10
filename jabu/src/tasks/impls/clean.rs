use crate::{
    tools::JavaHome,
    args::parser::ParsedArguments,
    tasks::{
        TaskError,
        JabuTask,
        TaskResult
    }
};
use jabu_config::model::JabuProject;

#[derive(Default)]
pub struct CleanTask;

impl JabuTask for CleanTask {
    fn execute(&self, _: Vec<String>, _: Option<ParsedArguments>, jabu_config: &JabuProject, _: &JavaHome) -> TaskResult {
        match std::fs::remove_dir_all(&jabu_config.fs_schema.target) {
            Ok(_) => (),
            Err(e) => {
                return Err(TaskError::IOError(e))
            }
        }

        match std::fs::create_dir_all(&jabu_config.fs_schema.target) {
            Ok(_) => (),
            Err(e) => {
                return Err(TaskError::IOError(e))
            }
        }
        Ok(())
    }

    fn description(&self) -> String {
        "Empties the target directory of the project.".to_string()
    }
}
