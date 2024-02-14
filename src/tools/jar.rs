use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, PartialEq, Clone)]
pub struct JarToolConfig {
    pub output_file: String,
    pub manifest_location: Option<PathBuf>,
    pub contents: HashMap<String, Vec<String>>,
}

impl Default for JarToolConfig {
    fn default() -> Self {
        Self {
            output_file: String::default(),
            manifest_location: None,
            contents: HashMap::new(),
        }
    }
}

impl Into<Vec<String>> for JarToolConfig {
    fn into(self) -> Vec<String> {
        let mut args = vec!["--create".to_string()];
        
        args.push("--file".to_string());
        args.push(self.output_file);

        if let Some(manifest_location) = self.manifest_location {
            args.push("--manifest".to_string());
            args.push(manifest_location.to_string_lossy().to_string());
        }

        self.contents.into_iter()
            .for_each(|(base_location, targets)| {
                args.push("-C".to_string());
                args.push(base_location);
                args.extend(targets);
            });

        args
    }
}

impl JarToolConfig {
    pub fn new(output_file: String, target_classes_location: String) -> Self {
        let mut contents = HashMap::new();
        contents.insert(target_classes_location, vec![".".to_string()]);
        Self {
            output_file,
            contents,
            manifest_location: None,
        }
    }

    /// Turns the `JarToolConfig` into a vector of arguments to 
    /// be given to the jar cli tool.
    pub fn into_args(self) -> Vec<String> {
        self.into()
    }
}
