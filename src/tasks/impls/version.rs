use crate::tasks::Task;

#[derive(Debug, Default)]
pub struct VersionTask;

impl Task for VersionTask {
    fn execute(&self, args: Vec<String>) -> crate::tasks::TaskResult {
        println!("Version: {}", crate::VERSION);
        Ok(())
    }

    fn description(&self) -> String {
        "Displays the version of Jabu".to_string()
    }
}
