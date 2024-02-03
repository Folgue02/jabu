use crate::tasks::Task;

#[derive(Debug)]
pub struct VersionTask;

impl Default for VersionTask {
    fn default() -> Self {
        Self
    }
}

impl Task for VersionTask {
    fn execute(&self, args: Vec<String>) -> crate::tasks::TaskResult {
        println!("Version: {}", crate::VERSION);
        Ok(())
    }

    fn description(&self) -> String {
        "Displays the version of Jabu".to_string()
    }
}
