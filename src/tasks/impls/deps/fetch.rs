use crate::{
    args::parser::ParsedArguments,
    config::JabuConfig,
    tasks::{JabuTask, TaskError, TaskResult},
    tools::JavaHome,
};
use jaburepo::repository::*;
use reqwest::blocking::get;

#[derive(Debug, PartialEq, Default)]
pub struct FetchDepsTask;

impl JabuTask for FetchDepsTask {
    fn execute(
        &self,
        _: Vec<String>,
        _: Option<ParsedArguments>,
        jabu_config: &JabuConfig,
        _: &JavaHome,
    ) -> TaskResult {
        let home_dir = if cfg!(windows) {
            std::env::var("USERPROFILE").unwrap()
        } else {
            std::env::var("HOME").unwrap()
        };
        let repo = Repository::default();
        let remote_repository = RemoteRepository::default(); // TODO CHECK ON OTHER REPOSITORIES
        let artifacts = Self::artifacts_of_dependencies(jabu_config);
        let missing_artifacts: Vec<&Artifact> = artifacts
            .iter()
            .filter(|artifact| !repo.exists(&artifact))
            .collect();

        println!("Fetching dependencies to local repository located at {}", repo.base_path().to_string_lossy());
        Self::fetch_missing_dependencies(&remote_repository, &repo, &missing_artifacts)?;
        Ok(())
    }

    fn description(&self) -> String {
        "Fetches the project's dependencies.".to_string()
    }
}

impl FetchDepsTask {
    /// Turns the specified dependencies into artifacts.
    fn artifacts_of_dependencies(jabu_config: &JabuConfig) -> Vec<Artifact> {
        jabu_config
            .dependencies
            .remote
            .iter()
            .map(|dep| dep.clone().into())
            .collect()
    }

    fn fetch_missing_dependencies(
        remote_repository: &RemoteRepository,
        repository: &Repository,
        missing_deps: &Vec<&Artifact>,
    ) -> Result<(), TaskError> {
        Ok(missing_deps.iter().try_for_each(|dep| {
            repository.recursive_save_from_remote(dep, remote_repository, |pom, jar| {
                println!("Fetching dependency:\n\tFetching pom: {pom}\n\tFetching jar: {jar}")
            })
        })?)
    }
}
