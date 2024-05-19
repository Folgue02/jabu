use crate::{
    args::parser::ParsedArguments,
    tasks::{JabuTask, TaskError, TaskResult},
    tools::JavaHome,
};
use home::home_dir;
use jabu_config::model::{ArtifactSpec, JabuProject};
use jaburepo::repository::Repository;
use std::path::PathBuf;

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
        // TODO: Allow specifying the jaburepo directory with env vars
        let proj_dir = std::env::current_dir()?;
        let jabu_repo =
            jaburepo::repository::Repository::new(home_dir().unwrap().join(".jaburepo"));

        // Split into missing and found
        let local_found_deps: Vec<&ArtifactSpec> = jabu_config
            .dependencies
            .remote
            .iter()
            .filter(|dep| jabu_repo.exists(dep))
            .collect();

        let missing_deps: Vec<&ArtifactSpec> = jabu_config
            .dependencies
            .remote
            .iter()
            .filter(|dep| !jabu_repo.exists(dep))
            .collect();

        if !local_found_deps.is_empty() {
            println!("Copying local dependencies {}...", local_found_deps.len());
            copy_dependencies_from_local_repo(
                &local_found_deps,
                &jabu_repo,
                proj_dir.join(&jabu_config.fs_schema.lib),
            )?;
        }

        if !missing_deps.is_empty() {
            let url = match std::env::var("JABU_REMOTE_REPO") {
                Ok(url) => url,
                Err(_) => "https://jabu-remote-repository.com".to_string(),
            };
            println!(
                "Fetching {} remote dependencies from '{}'...",
                missing_deps.len(),
                url
            );
            fetch_dependencies(&missing_deps, url, &jabu_repo)?;
            copy_dependencies_from_local_repo(
                &missing_deps,
                &jabu_repo,
                proj_dir.join(&jabu_config.fs_schema.lib),
            )?;
        }

        Ok(())
    }

    fn description(&self) -> String {
        "Fetches the project's dependencies.".to_string()
    }
}

fn copy_dependencies_from_local_repo(
    deps: &Vec<&ArtifactSpec>,
    repo: &Repository,
    lib_dir: PathBuf,
) -> TaskResult {
    deps.iter().try_for_each(|dep| {
        let from_repo_path = repo.jar_path(dep);
        println!(
            "Copying local artifact from\n\t'{}' to '{}'",
            from_repo_path.to_string_lossy().to_string(),
            lib_dir.to_string_lossy().to_string()
        );
        match std::fs::copy(&from_repo_path, lib_dir.join(dep.to_string())) {
            // TODO: Continue here
            Err(e) => {
                return Err::<(), TaskError>(TaskError::from_io_error(
                    e,
                    format!(
                        "Couldn't copy '{}' to '{}'.",
                        from_repo_path.to_string_lossy().to_string(),
                        lib_dir.to_string_lossy().to_string()
                    ),
                ))
            }
            _ => (),
        }
        Ok::<(), TaskError>(())
    })?;
    println!("Local dependencies copied.");
    Ok(())
}

fn fetch_dependencies(
    deps: &Vec<&ArtifactSpec>,
    url: impl Into<String>,
    repo: &Repository,
) -> TaskResult {
    let url = url.into();

    deps.iter().try_for_each(|dep| {
        // TODO: Use something to create urls
        let jar_url = format!(
            "{url}/api/get/{}/{}/{}/jar",
            dep.author, dep.artifact_id, dep.version
        );
        let jaburon_url = format!(
            "{url}/api/get/{}/{}/{}/jaburon",
            dep.author, dep.artifact_id, dep.version
        );
        let repo_jar_path = repo.jar_path(dep);

        println!("==> FETCHING artifact {}", dep);

        println!(
            "Fetching artifact from {} to {}...",
            jar_url,
            repo_jar_path.to_string_lossy()
        );
        let jar_res = reqwest::blocking::get(jar_url)?;
        // ERROR: No such file or directory.

        println!("Fetching jaburon from {}...", jaburon_url);
        let jaburon_res = reqwest::blocking::get(jaburon_url)?;

        println!("Saving artifact {}...", dep);
        repo.save_artifact(dep, &jar_res.bytes()?, &jaburon_res.bytes()?)?;

        Ok::<(), TaskError>(())
    })?;

    Ok(())
}
