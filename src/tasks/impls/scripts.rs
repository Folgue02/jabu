use crate::{tasks::JabuTask, args::{parser::ParsedArguments, options::{Options, ParOptionBuilder}}};
use std::{ffi::OsStr, path::PathBuf};
pub struct ScriptsTask;

impl Default for ScriptsTask {
    fn default() -> Self {
        Self
    }
}

impl JabuTask for ScriptsTask {
    fn execute(&self, args: Vec<String>, jabu_config: &crate::config::JabuConfig, java_home: &crate::tools::JavaHome) -> crate::tasks::TaskResult {
        let parsed_arguments = ParsedArguments::new_with_options(args, &ScriptsTask::get_options()).unwrap();
        let script_paths = ScriptsTask::get_scripts(jabu_config);

        if parsed_arguments.get_option_value("list").is_some() {
            script_paths.iter()
                .for_each(|path| println!("Script: {path:?}"));
            return Ok(())
        }



        Ok(())
    }

    fn description(&self) -> String {
        "Manages the project's scripts.".to_string()
    }
}

impl ScriptsTask {
    fn get_scripts(jabu_config: &crate::config::JabuConfig) -> Vec<PathBuf> {
        walkdir::WalkDir::new(&jabu_config.fs_schema.scripts)
            .into_iter()
            .filter_map(|e| match e {
                Ok(entry) => Some(entry),
                Err(_) => None,
            })
            .filter(|e| e.file_type().is_file())
            .map(|e| e.path().to_path_buf())
            .filter(|e| e.extension().unwrap_or_default() == "lua")
            .collect::<Vec<PathBuf>>()
    }
    fn get_options() -> Options {
        let mut options = Options::default();
        options.add_option(
            ParOptionBuilder::default()
                .name("list")
                .short('l')
                .has_arg(false)
                .description("Lists all available tasks of the project.")
                .build()
        );

        options
    }
}

