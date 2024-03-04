use crate::{
    utils::FSNodeType,
    tasks::JabuTask,
    tools::{JavaHome, JavacConfig}, utils,
    args::parser::ParsedArguments,
};

#[derive(Default)]
pub struct BuildJabuTask {}

impl JabuTask for BuildJabuTask {
    fn description(&self) -> String {
        "Builds the current project.".to_string()
    }

    fn execute(
        &self,
        _: Vec<String>,
        _: Option<ParsedArguments>,
        jabu_config: &crate::config::JabuConfig,
        java_home: &JavaHome,
    ) -> crate::tasks::TaskResult {
        
        let sources = jabu_config.fs_schema.get_java_sources();

        println!("Sources to compile: ");
        sources
            .iter()
            .enumerate()
            .for_each(|(index, source)| println!("{}: {source:?}", index + 1));
        println!("");

        let mut javac_config = JavacConfig::new(
            sources.iter().map(|source| source.to_string_lossy().to_string()).collect(),
            Some(jabu_config.fs_schema.target_classes()),
            Some(jabu_config.java_config.clone()),
        );
        
        // Add the jars under the lib directory of the project.
        javac_config.classpath = crate::utils::walkdir_find(
                &jabu_config.fs_schema.lib,
                |entry| entry.extension().unwrap_or_default() == "jar",
                &[FSNodeType::File, FSNodeType::SymLink]
            )
            .iter()
            .map(|jar_file| jar_file.to_string_lossy().to_string())
            .collect();

        let javac_args = javac_config.into_args();
        let javac_path = java_home.get_javac().clone().unwrap().to_string_lossy().to_string();

        let exit_status = match utils::exec_cmd(&javac_path, javac_args) {
            Ok(exit_status) => {
                if let Some(code) = exit_status.code() {
                    code
                } else {
                    return Err(crate::tasks::TaskError::CommandFailed{
                        command:"javac".to_string(), 
                        description: "Command has no exit code (probably due to a SIGINT)".to_string()
                    })
                }
            }
            Err(e) => {
                // i.e. The invoked binary doesn't exist.
                return Err(crate::tasks::TaskError::CommandFailed{command: "javac".to_string(), description: e.to_string()})
            }
        };

        if exit_status != 0 {
            Err(
                crate::tasks::TaskError::CommandFailed{
                    command: "javac".to_string(), 
                    description: exit_status.to_string()
                }
            )
        } else {
            Ok(())
        }
    }

    fn required_tools(&self) -> &[&'static str] {
        &["javac"]
    }
}
