use crate::tasks::{JabuTask, JabuTaskDependencySpec};
use crate::config::JabuConfig;
use crate::tools::{JavaHome, JavaToolConfig, JavaExecTarget};
use std::collections::HashMap;
use crate::args::options::{Options, ParOptionBuilder};
use crate::args::parser::ParsedArguments;

#[derive(Debug, Default)]
pub struct Run;

impl JabuTask for Run {
    fn description(&self) -> String {
        "Runs the current project".to_string()
    }

    fn execute(&self, _: Vec<String>, parsed_args: Option<ParsedArguments>, jabu_config: &JabuConfig, java_home: &JavaHome) -> crate::tasks::TaskResult {
        let parsed_args = parsed_args.unwrap();
        
        let main_class: &str = if let Some(Some(main_class)) = parsed_args.get_option_value("main-class") {
            main_class
        } else if let Some(main_class) = jabu_config.properties.get("Main-Class") {
            main_class.as_str()
        } else {
            return Err(crate::tasks::TaskError::Generic("The project it's either not executable, or doesn't contain a 'Main-Class' key in the properties in the jabu config.".to_string()));
        };
        
        // Declare the classpath, which should contain the 'target' dir, as well
        // as the project's libraries (jars).
        let mut classpath = vec![jabu_config.fs_schema.target_classes()];
        let jars: Vec<String> = crate::utils::walkdir_find(
            &jabu_config.fs_schema.lib,
            |entry| entry.extension().unwrap_or_default() == "jar",
            &[crate::utils::FSNodeType::File, crate::utils::FSNodeType::SymLink]
        ).iter()
            .map(|entry| entry.to_string_lossy().to_string())
            .collect();
        classpath.extend(jars);

        let java_tool_config = JavaToolConfig::new(
            JavaExecTarget::MainClass(main_class.to_string()),
            classpath,
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

    fn required_tools(&self) -> &[&'static str] {
        &["java", "javac"]
    }

    fn options(&self) -> Option<Options> {
        let mut options = Options::default();
        options.add_option(
            ParOptionBuilder::default()
                .name("main-class")
                .short('c')
                .description("Specify which class to run.")
                .has_arg(true)
                .required(false)
                .build()
        );
        Some(options)
    }
}
