use crate::{
    args::{
        options::{Options, ParOption, ParOptionBuilder},
        parser::{InvalidArgError, ParsedArguments},
    },
    config::{java::ProjectType, JabuConfig, JABU_FILE_NAME},
    tasks::*,
};
use std::{collections::HashSet, path::Path};

#[derive(Debug, Default)]
pub struct NewProjectTask;

impl Task for NewProjectTask {
    fn description(&self) -> String {
        "Creates a new project.".to_string()
    }

    fn execute(&self, args: Vec<String>, parsed_args: Option<ParsedArguments>) -> TaskResult {
        // Safe to unwrap, since the trait method `Task::options` returns `Some()`
        let parsed_args = parsed_args.unwrap();

        // Safely unwrap since if this options were missing it would have been caught
        // while parsing with the options.
        let new_project_name = parsed_args
            .get_option_value("name")
            .unwrap()
            .as_ref()
            .unwrap()
            .as_str();
        let new_project_path =
            Path::new(std::env::current_dir().unwrap().as_os_str()).join(new_project_name);
        let project_type = match ProjectType::try_from(
            parsed_args
                .get_option_value("project-type")
                .unwrap()
                .as_ref()
                .unwrap()
                .as_str(),
        ) {
            Ok(project_type) => project_type,
            Err(_) => {
                let mut errors = HashSet::new();
                errors.insert(InvalidArgError::InvalidOptionValue {
                    option_name: "project-type".to_owned(),
                    error_msg: "Invalid type given.".to_string(),
                });
                return Err(TaskError::InvalidArguments(errors));
            }
        };

        match std::fs::create_dir(new_project_path.clone()) {
            Err(e) => return Err(TaskError::IOError(e)),
            _ => (),
        }

        let project_config = JabuConfig::default_of_name(new_project_name, project_type);

        // Create the directories for the project
        let project_creation_result = project_config.fs_schema.create(&new_project_path.clone().to_string_lossy().to_string());
        match project_creation_result {
            Ok(_) => {
                ()
            }
            Err(e) => return Err(TaskError::IOError(e)),
        }

        // Write the project's config into a jabu file.
        let jabu_file_path = new_project_path.join(JABU_FILE_NAME).to_string_lossy().to_string();
        match std::fs::write(jabu_file_path, ron::ser::to_string_pretty(&project_config, ron::ser::PrettyConfig::default()).unwrap()) {
            Ok(_) => {
                println!("Project created.");
                Ok(())
            }
            Err(e) => Err(TaskError::IOError(e)),
        }
    }

    fn options(&self) -> Option<Options> {
        let mut options = Options::default();
        options.add_option(
            ParOptionBuilder::default()
                .name("name")
                .short('n')
                .description("Name of the new project.")
                .required(true)
                .has_arg(true)
                .build(),
        );

        options.add_option(
            ParOptionBuilder::default()
                .name("project-type")
                .short('t')
                .has_arg(true)
                .description("Defines the type of project to be created.")
                .default_value("binary".to_string())
                .build(),
        );

        Some(options)
    }
}
