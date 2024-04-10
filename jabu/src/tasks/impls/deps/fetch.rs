use crate::{
    args::parser::ParsedArguments,
    tasks::{JabuTask, TaskResult},
    tools::JavaHome,
};
use jabu_config::model::JabuProject;

#[derive(Debug, PartialEq, Default)]
pub struct FetchDepsTask;

impl JabuTask for FetchDepsTask {
    fn execute(
        &self,
        _: Vec<String>,
        _: Option<ParsedArguments>,
        jabu_config: &JabuProject,
        _: &JavaHome,
    ) -> TaskResult {
        todo!()
    }

    fn description(&self) -> String {
        "Fetches the project's dependencies.".to_string()
    }
}
