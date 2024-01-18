use crate::tasks::Task;

pub struct Run {
}

impl Task for Run {
    fn description(&self) -> String {
        "Runs the current project".to_string()
    }

    fn execute(&self, args: Vec<String>) -> crate::tasks::TaskResult {
        todo!()
    }
}
