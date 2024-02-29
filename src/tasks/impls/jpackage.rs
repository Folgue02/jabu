use crate::{
    args::{
        options::{
            Options,
            ParOptionBuilder,
        },
        parser::{
            ParsedArguments,
            InvalidArgError
        },
    },
    config::JabuConfig,
    tools::{
        JavaHome,
        JPackageToolConfig,
        JavadocToolConfig,
        JavaVisibilityLevel,
    },
    tasks::{
        JabuTaskDependencySpec,
        TaskResult,
        TaskError,
        JabuTask
    },
    utils::exec_cmd
};

use std::path::PathBuf;

#[derive(Default)]
pub struct JPackageTask;

impl JabuTask for JPackageTask {
    fn description(&self) -> String {
        "Generates a self-contained application of the project.".to_string()
    }

    fn execute(&self, args: Vec<String>, jabu_config: &JabuConfig, java_home: &JavaHome) -> TaskResult {
        let parsed_args = match ParsedArguments::new_with_options(args, &Self::get_options()) {
            Ok(args) => args,
            Err(e) => return Err(TaskError::InvalidArguments(e)),
        };
        let main_class = if let Some(main_class) = jabu_config.properties.get("Main-Class") {
            main_class
        } else {
            return Err(
                TaskError::Generic(
                    "Cannot create a self-contained application on a non executable project.".to_string()
                )
            )
        };
        let output_type = parsed_args.get_option_value("output-type").unwrap_or(&None);
        let input_jar_location = PathBuf::from(jabu_config.fs_schema.target_bin()).join(format!("{}.jar", jabu_config.display_name()));
        let jpackage_config = JPackageToolConfig::new(
            input_jar_location,
            jabu_config.header.project_name.clone(),
            main_class.clone(),
            jabu_config.fs_schema.target_self_contained(),
            output_type.clone() // TODO: Check how to make the clone not necessary
        );
        let jpackage_path = &java_home.get_jpackage().clone().unwrap().to_string_lossy().to_string();

        let exit_status = match exec_cmd(jpackage_path, jpackage_config.try_into_args().unwrap()) {
            Ok(exit_status) => {
                if let Some(code) = exit_status.code() {
                    code
                } else {
                    return Err(crate::tasks::TaskError::CommandFailed{
                        command:"jpackage".to_string(), 
                        description: "Command has no exit code (probably due to a SIGINT)".to_string()
                    })
                }
            }
            Err(e) => {
                // i.e. The invoked binary doesn't exist.
                return Err(crate::tasks::TaskError::CommandFailed{command: "jpackage".to_string(), description: e.to_string()})
            }
        };

        if exit_status != 0 {
            Err(
                crate::tasks::TaskError::CommandFailed{
                    command: "jpackage".to_string(), 
                    description: exit_status.to_string()
                }
            )
        } else {
            Ok(())
        }
    }

    fn get_dependency_task_specs(&self) -> JabuTaskDependencySpec {
        let mut spec = std::collections::HashMap::new();
        spec.insert("jar".to_string(), Vec::new());

        JabuTaskDependencySpec::new(spec)
    }
}

impl JPackageTask {
    fn get_options() -> Options {
        let mut ops = Options::default();
        ops.add_option(
            ParOptionBuilder::default()
                .name("output-type")
                .short('t')
                .description("Type of the output application (platform dependant)")
                .required(false)
                .has_arg(true)
                .build()
        );

        ops
    }
}
