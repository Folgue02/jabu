use crate::tasks::{Task, JabuTask, JabuTaskDependencySpec};
use crate::config::JabuConfig;
use crate::tools::{JavaHome, JavaToolConfig, JavaExecTarget};
use std::collections::HashMap;
use crate::args::options::{Options, ParOptionBuilder, ParOption};
use crate::args::parser::ParsedArguments;

#[derive(Debug)]
pub struct Run {
}

impl Default for Run {
    fn default() -> Self {
        Self {}
    }
}

impl Run {
    fn get_options() -> Options {
        let mut options = Options::default();
        options.add_option(
            ParOptionBuilder::default()
                .name("main-class")
                .short('c')
                .required(false)
                .build()
        );
        options
    }
}

impl JabuTask for Run {
    fn description(&self) -> String {
        "Runs the current project".to_string()
    }

    fn execute(&self, args: Vec<String>, jabu_config: &JabuConfig, java_home: &JavaHome) -> crate::tasks::TaskResult {
        let parsed_args = 
            match ParsedArguments::new_with_options(args, &Run::get_options()) {
                Ok(p) => p,
                Err(e) => return Err(crate::tasks::TaskError::InvalidArguments(e))
            };

        
        let main_class: &str = if let Some(Some(main_class)) = parsed_args.get_option_value("main-class") {
            main_class
        } else if let Some(main_class) = jabu_config.properties.get("Main-Class") {
            main_class.as_str()
        } else {
            return Err(crate::tasks::TaskError::Generic("The project it's either not executable, or doesn't contain a 'Main-Class' key in the properties in the jabu config.".to_string()));
        };
        let java_tool_config = JavaToolConfig::new(
            JavaExecTarget::MainClass(main_class.to_string()),
            vec![jabu_config.fs_schema.target.to_string()],
            Vec::new()
        );
        if let Some(java_path) = java_home.get_java() {
            match crate::utils::exec_cmd(java_path.to_str().unwrap(), java_tool_config.into_args()) {
                Err(e) => {
                    return Err(crate::tasks::TaskError::IOError(e));
                }
                _ => ()
            }
        }
        Ok(())
    }

    fn get_dependency_task_specs(&self) -> crate::tasks::JabuTaskDependencySpec {
        let mut specs = HashMap::new();
        specs.insert("build".to_string(), Vec::new());
        JabuTaskDependencySpec::new(specs)
    }
}
