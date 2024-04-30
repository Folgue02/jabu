use crate::{
    args::parser::ParsedArguments,
    tasks::{JabuTask, TaskError, TaskResult},
    tools::JavaHome,
};
use jabu_config::{
    fsutils::libs,
    model::{ArtifactSpec, JabuProject},
};
use prettytable::{color, Attr, Cell, Row};
use std::collections::HashMap;

#[derive(Default)]
pub struct ListDepsTask;

impl JabuTask for ListDepsTask {
    fn execute(
        &self,
        _: Vec<String>,
        _: Option<ParsedArguments>,
        jabu_config: &JabuProject,
        _: &JavaHome,
    ) -> TaskResult {
        let dep_names: Vec<String> = libs(None, jabu_config)
            .iter()
            .map(|lib| {
                lib.file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap_or("-----")
                    .to_string()
            })
            .collect();

        if jabu_config.dependencies.local.is_empty() && jabu_config.dependencies.remote.is_empty() {
            println!("==> No local/remote dependencies specified in the jabu file.");
        } else {
            Self::list_dependencies(jabu_config).expect("Error while listing dependencies");
        }

        Ok(())
    }

    fn description(&self) -> String {
        "List all dependencies.".to_string()
    }
}

impl ListDepsTask {
    fn list_dependencies(jabu_config: &JabuProject) -> TaskResult {
        let local_dependencies_status = Self::check_local_dependencies(jabu_config)?;

        if !jabu_config.dependencies.local.is_empty() {
            println!("==> LOCAL DEPENDENCIES");
            local_dependencies_status
                .iter()
                .enumerate()
                .for_each(|(index, (artifact, existence))| {
                    println!(
                        "\t{} -> {} : {}",
                        index + 1,
                        artifact.to_string(),
                        if *existence { "Found" } else { "Not found" }
                    );
                });
        } else {
            println!("==> No local dependencies specified in the jabu file.");
        }

        if !jabu_config.dependencies.remote.is_empty() {
            // TODO: Remote dependencies
            println!("==> REMOTE DEPENDENCIES");
        } else {
            println!("==> No remote dependencies specified in the jabu file.");
        }
        Ok(())
    }

    fn check_local_dependencies(
        jabu_config: &JabuProject,
    ) -> Result<HashMap<ArtifactSpec, bool>, TaskError> {
        // Read dir and turn file names into ArtifactSpec (parse the `file_stem`)
        // When a file cannot be read, or a file_stem cannot be parsed, it isn't
        // included.
        let lib_files_as_artifactspecs: Vec<ArtifactSpec> =
            std::fs::read_dir(&jabu_config.fs_schema.lib)?
                .filter_map(|entry| entry.ok())
                .map(|entry| {
                    entry
                        .path()
                        .file_stem()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string()
                })
                .filter_map(|entry| ArtifactSpec::try_from(entry.as_str()).ok())
                .collect();

        Ok(jabu_config
            .dependencies
            .local
            .iter()
            .map(|artifact| {
                (
                    artifact.clone(),
                    lib_files_as_artifactspecs.contains(&artifact),
                )
            })
            .collect::<HashMap<ArtifactSpec, bool>>())
    }
}
