use std::{
    path::{Path, PathBuf},
    collections::HashMap
};
use crate::{
    utils::{walkdir_find, FSNodeType, exec_cmd},
    config::{
        JabuConfig,
        java::JarManifest
    },
    tools::{JavaHome, JarToolConfig},
    tasks::{
        TaskError,
        JabuTaskDependencySpec,
        JabuTask, 
        TaskResult
    }
};

#[derive(Default)]
pub struct JarTask;

impl JabuTask for JarTask {
    fn description(&self) -> String {
        "Creates a jar containing the compiled classes of the project.".to_string()
    }

    fn execute(&self, _: Vec<String>, jabu_config: &JabuConfig, java_home: &JavaHome) -> TaskResult {
        let mut jar_tool_config = JarToolConfig::new(
            Path::new(&jabu_config.fs_schema.target_bin())
                .join(jabu_config.display_name() + ".jar")
                .to_string_lossy()
                .to_string(),
            jabu_config.fs_schema.target_classes()
        );
        let jar_path = java_home.get_jar().as_ref().unwrap().to_string_lossy().to_string();
        let manifest_path: PathBuf;
        let status_code: i32;

        // Create the target/bin dir + Create the manifest
        std::fs::create_dir_all(&jabu_config.fs_schema.target_bin())?;
        manifest_path = JarTask::write_manifest(&jabu_config)?; 

        jar_tool_config.manifest_location = Some(manifest_path);

        status_code = match exec_cmd(&jar_path, jar_tool_config.into_args()) {
            Ok(exit_code) => {
                if let Some(status_code) = exit_code.code() {
                    status_code
                } else {
                    return Err(
                        TaskError::CommandFailed {
                            command: jar_path,
                            description: "Command has no exit code (probably due to a SIGINT)".to_string()
                        }
                    )
                }
            }
            Err(e) => {
                return Err(
                    TaskError::CommandFailed {
                        command: jar_path,
                        description: e.to_string()
                    }
                );
            }
        };

        if status_code != 0 {
            Err(
                TaskError::CommandFailed {
                    command: jar_path,
                    description: status_code.to_string()
                }
            )
        } else {
            Ok(())
        }
    }
    fn get_dependency_task_specs(&self) -> JabuTaskDependencySpec {
        let mut specs = HashMap::new();
        specs.insert("build".to_string(), Vec::new());

        JabuTaskDependencySpec::new(specs)
    }

    fn required_tools(&self) -> &[&'static str] {
        &["javac", "jar"]
    }
}

impl JarTask {
    /// Writes the manifest into the `target/bin` directory with the name `MANIFEST.MF`,
    /// if `target/bin` directory didn't exist, it will be created.
    fn write_manifest(jabu_config: &JabuConfig) -> std::io::Result<PathBuf> {
        let manifest: JarManifest = JarManifest::from(jabu_config.properties.clone());
        let manifest_path = Path::new(&jabu_config.fs_schema.target_bin()).join("MANIFEST.MF");
        std::fs::create_dir_all(jabu_config.fs_schema.target_bin())?;
        manifest.write_to_file(manifest_path.as_path())?;
        Ok(manifest_path)
    }
}
