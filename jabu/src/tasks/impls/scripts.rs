use crate::{
    args::{
        options::{Options, ParOptionBuilder},
        parser::ParsedArguments,
    },
    tasks::JabuTask,
};
use jabu_config::model::JabuProject;
use rhai::{packages::Package, Array, Dynamic, Engine, EvalAltResult, Scope};
use rhai_fs::FilesystemPackage;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

#[derive(Default)]
pub struct ScriptsTask;

impl JabuTask for ScriptsTask {
    fn execute(
        &self,
        args: Vec<String>,
        parsed_args: Option<ParsedArguments>,
        jabu_config: &JabuProject,
        java_home: &crate::tools::JavaHome,
    ) -> crate::tasks::TaskResult {
        let parsed_arguments = parsed_args.unwrap();
        let script_paths = ScriptsTask::get_scripts(jabu_config);
        let mut results = HashMap::new();

        if parsed_arguments.get_option_value("list").is_some() {
            script_paths
                .iter()
                .for_each(|path| println!("Script: {path:?}"));
            Ok(())
        } else if parsed_arguments.arg_list.is_empty() {
            Err(crate::tasks::TaskError::Generic(
                "You didn't specify any scripts to be executed.".to_string(),
            ))
        } else {
            parsed_arguments.arg_list.iter().for_each(|script_name| {
                let script_path = Self::get_script_path(jabu_config, script_name);
                results.insert(script_name, Self::run_script(jabu_config, script_path));
            });

            // Indicates if any of the scripts have returned
            // an error.
            let mut errors = false;

            // Print script execution results and check if all scripts
            // have been succesfully executed.
            results
                .iter()
                .enumerate()
                .for_each(|(index, (script_name, script_result))| {
                    let index = index + 1;
                    let msg_prefix = format!("[{index}#{script_name}]:");

                    match script_result {
                        Ok(_) => println!("{msg_prefix} Done."),
                        Err(e) => {
                            errors = true;
                            match e.unwrap_inner() {
                                EvalAltResult::Return(_, _) | EvalAltResult::Exit(_, _) => {
                                    // Everything went ok.
                                    println!("{msg_prefix} Done.");
                                }
                                EvalAltResult::ErrorParsing(e_type, pos) => {
                                    eprintln!("{msg_prefix} Parsing error at position: {pos}");
                                    eprintln!("{msg_prefix} Error type: {e_type}");
                                }
                                EvalAltResult::ErrorSystem(msg, internal_error) => {
                                    eprintln!("{msg_prefix} System error: {msg}");
                                    eprintln!("{msg_prefix} Internal error: {internal_error}");
                                }
                                _ => {
                                    eprintln!("{msg_prefix} Error while executing the script: {e}");
                                }
                            }
                        }
                    }
                });
            if errors {
                Err(crate::tasks::TaskError::Generic(
                    "One or more scripts have failed.".to_string(),
                ))
            } else {
                Ok(())
            }
        }
    }

    fn description(&self) -> String {
        "Manages the project's scripts.".to_string()
    }

    fn options(&self) -> Option<Options> {
        let mut options = Options::default();
        options.add_option(
            ParOptionBuilder::default()
                .name("list")
                .short('l')
                .has_arg(false)
                .description("Lists all available tasks of the project.")
                .build(),
        );

        Some(options)
    }
}

impl ScriptsTask {
    /// Returns the contents of the given script as a string. The script_name
    /// is not supposed to be the path to the script, instead, it is its *middle name*,
    /// meaning that if there is a script in `scripts/my_script.rhai`, this script's name
    /// would be `my_script`.
    fn read_script(jabu_config: &JabuProject, script_name: &str) -> std::io::Result<String> {
        std::fs::read_to_string(
            Path::new(&jabu_config.fs_schema.scripts).join(format!("{script_name}.rhai")),
        )
    }

    /// Returns the path to the script referred to by the script name.
    ///
    /// # See
    /// [`Self::read_script`]
    fn get_script_path(jabu_config: &JabuProject, script_name: impl AsRef<str>) -> PathBuf {
        let script_name = script_name.as_ref();
        PathBuf::from(&jabu_config.fs_schema.scripts).join(format!("{script_name}.rhai"))
    }

    /// Returns all the paths to available scripts in the scripts folder.
    /// # Note
    /// If the scripts folder cannot be read, this function will return an
    /// empty Vec.
    fn get_scripts(jabu_config: &JabuProject) -> Vec<PathBuf> {
        walkdir::WalkDir::new(&jabu_config.fs_schema.scripts)
            .into_iter()
            .filter_map(|e| match e {
                Ok(entry) => Some(entry),
                Err(_) => None,
            })
            .filter(|e| e.file_type().is_file())
            .map(|e| e.path().to_path_buf())
            .filter(|e| e.extension().unwrap_or_default() == "rhai")
            .collect::<Vec<PathBuf>>()
    }

    /// Runs the script located at `path_to_script`, and returns the result of the execution.
    ///
    /// This function already does the binding of the DSL for the engine.
    fn run_script(
        jabu_config: &JabuProject,
        path_to_script: PathBuf,
    ) -> Result<(), Box<EvalAltResult>> {
        //let proj_cfg = ProjectConfig::new(jabu_config.clone(), std::env::current_dir().unwrap());
        let fs_package = FilesystemPackage::new();
        let mut engine = Engine::new();
        let mut scope = Scope::new();

        // Register types
        engine.build_type::<ProjectConfig>();

        // Register the filesystem package.
        fs_package.register_into_engine(&mut engine);

        // Bind the project configuration and other constants.
        scope.push_constant("VERSION", crate::VERSION);
        scope.push_constant(
            "proj_cfg",
            ProjectConfig {
                proj: jabu_config.clone(),
            },
        );

        engine.run_file_with_scope(&mut scope, path_to_script)
    }
}
use rhai::{CustomType, TypeBuilder};

#[derive(Debug, Clone, CustomType)]
#[rhai_type(extra = Self::build_extra)]
pub struct ProjectConfig {
    pub proj: JabuProject,
}

impl ProjectConfig {
    pub fn build_extra(builder: &mut TypeBuilder<Self>) {
        builder
            .with_name("ProjectConfig")
            .with_fn("lib_path", Self::lib_path)
            .with_fn("scripts_path", Self::scripts_path)

            .with_fn("target_path", Self::target_path)
            .with_fn("target_docs_path", Self::target_docs_path)
            .with_fn("target_bin_path", Self::target_bin_path)
            .with_fn("target_self_contained_path", Self::target_self_contained_path)
            .with_fn("local_dependencies", Self::local_dependencies)
            .with_fn("remote_dependencies", Self::remote_dependencies)

            .with_fn("source_path", Self::source_path)

            .with_fn("artifact_ident", Self::artifact_identifier)
        ;
    }

    pub fn artifact_identifier(&mut self) -> String {
        self.proj.header.to_string()
    }

    pub fn local_dependencies(&mut self) -> Array {
        self.proj.dependencies.local.iter()
            .map(|dep| Dynamic::from(dep.to_string()))
            .collect()
    }

    pub fn remote_dependencies(&mut self) -> Array {
        self.proj.dependencies.remote.iter()
            .map(|dep| Dynamic::from(dep.to_string()))
            .collect()
    }


    pub fn source_path(&mut self) -> String {
        self.proj.fs_schema.source.clone()
    }

    pub fn target_self_contained_path(&mut self) -> String {
        self.proj.fs_schema.target_self_contained().to_string_lossy().to_string()
    }

    pub fn lib_path(&mut self) -> String {
        self.proj.fs_schema.lib.clone()
    }

    pub fn scripts_path(&mut self) -> String {
        self.proj.fs_schema.scripts.clone()
    }

    pub fn target_path(&mut self) -> String {
        self.proj.fs_schema.target.clone()
    }

    pub fn target_bin_path(&mut self) -> String {
        self.proj.fs_schema.target_bin().to_string_lossy().to_string()
    }

    pub fn target_docs_path(&mut self) -> String {
        self.proj.fs_schema.target_docs().to_string_lossy().to_string()
    }
}
