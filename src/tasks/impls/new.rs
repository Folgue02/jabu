use crate::tasks::*;

pub struct RunTask {}

impl Task for RunTask {
    fn description(&self) -> String {
        "Creates a new project.".to_string()
    }

    fn execute(&self, args: Vec<String>) -> TaskResult {
        Ok(())
    }
}
