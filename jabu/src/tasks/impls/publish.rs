use crate::args::options::Options;
use crate::{
    args::{options::ParOptionBuilder, parser::ParsedArguments},
    tasks::{JabuTask, JabuTaskDependencySpec, TaskError, TaskResult},
    tools::JavaHome,
};
use jabu_config::model::{ArtifactSpec, JabuProject};
use jaburepo::repository::Repository;
use reqwest::blocking::multipart::{Form, Part};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

#[derive(Default)]
pub struct PublishTask;

impl JabuTask for PublishTask {
    fn description(&self) -> String {
        "Publish the current project to the repository.".to_string()
    }

    fn execute(
        &self,
        _: Vec<String>,
        options: Option<ParsedArguments>,
        jabu_config: &JabuProject,
        java_home: &JavaHome,
    ) -> TaskResult {
        let url = match std::env::var("JABU_REMOTE_REPO") {
            Ok(url) => url,
            Err(_) => "https://jabu-remote-repository.com".to_string(),
        };
        // TODO: PUBLISH-TASK
        // - Use PathBuf to join paths?
        // - Refactor the author_key variable's related code.
        let author_key = options.unwrap();
        let author_key = author_key.options.get("author-key");
        let proj_dir = std::env::current_dir().unwrap();
        let post_url = format!(
            "{url}/api/upload/{}/{}/{}/{}",
            jabu_config.header.author,
            jabu_config.header.project_name,
            jabu_config.header.version,
            author_key.unwrap().clone().unwrap()
        );

        let jaburon_path = proj_dir.join("jabu.ron");

        let mut files_content = Vec::new();

        // TODO: Refactor?
        let _ = [
            jabu_config.fs_schema.target_bin().join(
                ArtifactSpec::new(
                    &jabu_config.header.author,
                    &jabu_config.header.project_name,
                    &jabu_config.header.version,
                )
                .to_string()
                    + ".jar",
            ),
            jaburon_path,
        ]
        .iter()
        .try_for_each(|path| {
            let f = File::open(&path)?;
            let mut b_reader = BufReader::new(f);
            let mut contents = Vec::new();

            println!(
                "Reading from file '{}'...",
                path.to_string_lossy().to_string()
            );

            b_reader.read_to_end(&mut contents)?;
            files_content.push(contents);
            Ok::<(), std::io::Error>(())
        });

        let form = Form::new();

        let jaburon_part = Part::bytes(files_content.pop().unwrap());
        let jar_part = Part::bytes(files_content.pop().unwrap());

        let form = form.part("jaburon", jaburon_part).part("jar", jar_part);

        let client = reqwest::blocking::Client::new();
        let resp = client.post(post_url).multipart(form).send()?;

        if resp.status().is_success() {
            println!("Artifact published!");
            Ok(())
        } else {
            Err(TaskError::Generic(format!(
                "The artifact couldn't be published.\nThe server has returned {} as error code.",
                resp.status()
            )))
        }
    }

    fn options(&self) -> Option<Options> {
        let mut options = Options::default();
        options.add_option(
            ParOptionBuilder::default()
                .name("author-key")
                .short('k')
                .has_arg(true)
                .required(true)
                .build(),
        );

        Some(options)
    }

    fn get_dependency_task_specs(&self) -> JabuTaskDependencySpec {
        let mut specs = HashMap::new();

        specs.insert("jar".to_string(), Vec::new());

        JabuTaskDependencySpec::new(specs)
    }

    fn required_tools(&self) -> &[&'static str] {
        &["jar"]
    }
}
