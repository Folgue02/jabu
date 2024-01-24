use walkdir::WalkDir;

use crate::{
    config::JavaConfig,
    tasks::JabuTask,
    tools::{JavaHome, JavacConfig}, utils,
};

#[derive(Default)]
pub struct BuildJabuTask {}

impl JabuTask for BuildJabuTask {
    fn description(&self) -> String {
        "Builds the current project.".to_string()
    }

    fn execute(
        &self,
        args: Vec<String>,
        jabu_config: &crate::config::JabuConfig,
        java_home: &JavaHome,
    ) -> crate::tasks::TaskResult {
        
        let sources = WalkDir::new(&jabu_config.fs_schema.source)
            .into_iter()
            .filter_map(|e| match e {
                Ok(entry) => Some(entry),
                Err(_) => None,
            })
            .filter(|e| e.file_type().is_file())
            .map(|e| e.path().to_string_lossy().to_string())
            .filter(|e| e.ends_with(".java"))
            .collect::<Vec<String>>();

        println!("Sources to compile: ");
        sources
            .iter()
            .enumerate()
            .for_each(|(index, source)| println!("{index}: {source}"));
        println!("");

        let javac_args: Vec<String> = JavacConfig::new(
            sources,
            Some(jabu_config.fs_schema.target.to_string()),
            Some(jabu_config.java_config.clone()),
        )
        .into_args();

        let javac_path = java_home.get_javac().clone().unwrap().to_string_lossy().to_string();

        let exit_status = match utils::exec_cmd(&javac_path, javac_args) {
            Ok(exit_status) => {
                if let Some(code) = exit_status.code() {
                    code
                } else {
                    return Err(crate::tasks::TaskError::CommandFailed("javac".to_string(), "Command has no exit code (probably due to a SIGINT)".to_string()))
                }
            }
            Err(e) => {
                return Err(crate::tasks::TaskError::CommandFailed("javac".to_string(), e.to_string()))
            }
        };

        if exit_status != 0 {
            Err(crate::tasks::TaskError::CommandFailed("javac".to_string(), stringify!(exit_status).to_string()))
        } else {
            Ok(())
        }
    }
}
