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
        JavadocToolConfig,
        JavaVisibilityLevel,
    },
    tasks::{
        TaskResult,
        TaskError,
        JabuTask
    },
    utils::exec_cmd
};

#[derive(Default)]
pub struct JavadocTask;

impl JabuTask for JavadocTask {
    fn description(&self) -> String {
        "Generates the project's javadoc.".to_string()
    }

    fn execute(&self, args: Vec<String>, jabu_config: &JabuConfig, java_home: &JavaHome) -> TaskResult {
        let parsed_args = match ParsedArguments::new_with_options(args, &Self::get_options()) {
            Ok(args) => args,
            Err(e) => return Err(TaskError::InvalidArguments(e))
        };
        let sources = jabu_config.fs_schema.get_java_sources();
        let visibility_level = match JavaVisibilityLevel::try_from(
            parsed_args.get_option_value("visibility").unwrap().as_ref().unwrap().as_ref()) {
            Ok(vl) => vl,
            Err(_) => {
                let mut hs = std::collections::HashSet::new();
                hs.insert(
                    InvalidArgError::InvalidOptionValue {
                        option_name: "visibility".to_string(),
                        error_msg: "Invalid type given.".to_string()
                    }
                );
                return Err(TaskError::InvalidArguments(hs))
            }
        };

        if sources.is_empty() {
            eprintln!("No sources to parse.");
            return Ok(());
        } else {
            println!("Sources to parse: ");
            sources.iter()
                .enumerate()
                .for_each(|(index, path)| println!("{}: {}", index + 1, path.to_str().unwrap()));
        }

        let javadoc_config = JavadocToolConfig::new(
            sources.iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect(),
            Some(jabu_config.fs_schema.target_docs()),
            Some(jabu_config.java_config.clone()),
            visibility_level 
        );


        let cmd_result = exec_cmd(
            &java_home.get_javadoc().clone().unwrap().to_string_lossy().to_string(),
            javadoc_config.into_args()
        );

        let exit_code = match cmd_result {
            Ok(exit_status) => {
                if let Some(code) = exit_status.code() {
                    code
                } else {
                    return Err(crate::tasks::TaskError::CommandFailed {
                        command:"javadoc".to_string(), 
                        description: "Command has no exit code (probably due to a SIGINT)".to_string()
                    })
                }
            }
            Err(e) => {
                // i.e. The invoked binary doesn't exist.
                return Err(crate::tasks::TaskError::CommandFailed{command: "javadoc".to_string(), description: e.to_string()})
            }
        };

        if exit_code != 0 {
            Err(
                crate::tasks::TaskError::CommandFailed{
                    command: "javac".to_string(), 
                    description: exit_code.to_string()
                }
            )
        } else {
            Ok(())
        }
    }

    fn required_tools(&self) -> &[&'static str] {
        &["javadoc"]
    }
}

impl JavadocTask {
    fn get_options() -> Options {
        let mut options = Options::default();
        options.add_option(
            ParOptionBuilder::default()
                .name("visibility")
                .short('v')
                .required(false)
                .default_value("private".to_string())
                .build()
        );

        options
    }
}

