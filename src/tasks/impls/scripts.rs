use crate::{tasks::JabuTask, args::{parser::ParsedArguments, options::{Options, ParOptionBuilder}}, config::JabuConfig};
use std::{path::{PathBuf, Path}, collections::HashMap};
use pyo3::prelude::*;

#[derive(Default)]
pub struct ScriptsTask;

impl JabuTask for ScriptsTask {
    fn execute(
        &self,
        args: Vec<String>,
        parsed_args: Option<ParsedArguments>,
        jabu_config: &crate::config::JabuConfig,
        java_home: &crate::tools::JavaHome) -> crate::tasks::TaskResult {
        let parsed_arguments = parsed_args.unwrap();
        let script_paths = ScriptsTask::get_scripts(jabu_config);
        let mut results = HashMap::new();

        if parsed_arguments.get_option_value("list").is_some() {
            script_paths.iter()
                .for_each(|path| println!("Script: {path:?}"));
            Ok(())
        } else if parsed_arguments.arg_list.is_empty() {
            Err(crate::tasks::TaskError::Generic("You didn't specify any scripts to be executed.".to_string()))
        } else {
            pyo3::prepare_freethreaded_python();
            Python::with_gil(|py| {
                let jabu_module = PyModule::new(py, "jabu").unwrap();
                jabu_module.add_function(wrap_pyfunction!(crate::dslapi::prelude::print, jabu_module).unwrap()).unwrap();
                jabu_module.add_function(wrap_pyfunction!(crate::dslapi::prelude::get_version, jabu_module).unwrap()).unwrap();

                let sys_module = PyModule::import(py, "sys").unwrap();
                let modules: &pyo3::types::PyDict = sys_module.getattr("modules").unwrap().downcast().unwrap();

                modules.set_item("jabu", jabu_module).expect("Couldn't create Rust bindings for pyo3.");


                // Try to execute each script, and store the results
                // indexed by the script name.
                for script_name in parsed_arguments.arg_list {
                    let script_content = match ScriptsTask::read_script(jabu_config, &script_name) {
                        Ok(script_content) => script_content,
                        Err(e) => { 
                            results.insert(script_name, PyScriptExecutionResult::IOError(e));
                            continue;
                        }
                    };

                    println!("====> Executing script '{script_name}'");
                    // TODO: Create an enum and store the results 
                    // per enum variant? i.e. IOError, RuntimeError & Success? 
                    match py.run(&script_content, None, None) {
                        Ok(_) => results.insert(script_name.clone(), PyScriptExecutionResult::Success(0)),
                        Err(e) => results.insert(script_name.clone(), PyScriptExecutionResult::RuntimeError(e))
                    };
                    println!("====> Script '{}' done.", &script_name);
                }
            });
            let mut errors = false;

            // Print script execution results and check if all scripts 
            // have been succesfully executed.
            results.iter()
                .enumerate()
                .for_each(|(index, (script_name, script_result))| {
                    let index = index + 1;
                    match script_result {
                        PyScriptExecutionResult::RuntimeError(e) => {
                            let msg = format!("Runtime error: {e}");
                            eprintln!("[{index}#{script_name}]: {msg}");
                            errors = true;
                        }
                        PyScriptExecutionResult::IOError(e) => {
                            let msg = format!("Couldn't read : {e}");
                            eprintln!("[{index}#{script_name}]: {msg}");
                            errors = true;
                        }
                        PyScriptExecutionResult::Success(time) => {
                            let msg = format!("Took {time}ms");
                            println!("[{index}#{script_name}]: {msg}");
                        }
                    }
                });
            if errors {
                Err(crate::tasks::TaskError::Generic("One or more scripts have failed.".to_string()))
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
                .build()
        );

        Some(options)
    }
}

impl ScriptsTask {
    /// Returns the contents of the given script as a string. The script_name
    /// is not supposed to be the path to the script, instead, it is its *middle name*,
    /// meaning that if there is a script in `scripts/my_script.py`, this script's name
    /// would be `my_script`.
    fn read_script(jabu_config: &JabuConfig, script_name: &str) -> std::io::Result<String> {
        std::fs::read_to_string(
            Path::new(&jabu_config.fs_schema.scripts)
                .join(format!("{script_name}.py"))
        )
    }

    /// Returns all the paths to available scripts in the scripts folder.
    /// **If the scripts folder cannot be read, this function will return an 
    /// empty Vec**.
    fn get_scripts(jabu_config: &crate::config::JabuConfig) -> Vec<PathBuf> {
        walkdir::WalkDir::new(&jabu_config.fs_schema.scripts)
            .into_iter()
            .filter_map(|e| match e {
                Ok(entry) => Some(entry),
                Err(_) => None,
            })
            .filter(|e| e.file_type().is_file())
            .map(|e| e.path().to_path_buf())
            .filter(|e| e.extension().unwrap_or_default() == "py")
            .collect::<Vec<PathBuf>>()
    }
}

enum PyScriptExecutionResult {
    IOError(std::io::Error),
    RuntimeError(PyErr),
    Success(i32)
}

impl PyScriptExecutionResult {
    /// Checks if the result represents an error.
    pub fn is_error(&self) -> bool {
        match self {
            Self::IOError(_) | Self::RuntimeError(_) => true,
            _ => false
        }
    }
}
