use crate::{
    args::parser::ParsedArguments,
    tools::JavaHome,
    tasks::{
        Task,
        TaskResult
    }
};

#[derive(Debug, Default)]
pub struct HealthCheckTask;

impl Task for HealthCheckTask {
    fn execute(&self, _: Vec<String>, _: Option<ParsedArguments>) -> TaskResult {
        let java_home = JavaHome::new()?;

        java_home.get_tools();
        java_home.print_tool_availability_table();

        Ok(())
    }

    fn description(&self) -> String {
        "Perform a health check for the tools".to_string()
    }
}

