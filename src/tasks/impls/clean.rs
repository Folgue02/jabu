use crate::{config::JabuConfig, tools::JavaHome, tasks::{TaskError, JabuTask, TaskResult}};

#[derive(Default)]
pub struct CleanTask;

impl JabuTask for CleanTask {
    fn execute(&self, _: Vec<String>, jabu_config: &JabuConfig, _: &JavaHome) -> TaskResult {
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
