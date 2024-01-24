use std::path::PathBuf;

use crate::config::JavaConfig;

#[derive(Debug, PartialEq)]
pub struct JavacConfig {
    sources: Vec<String>,
    output_dir: Option<String>,
    java_config: Option<JavaConfig>,
}

impl Into<Vec<String>> for JavacConfig {
    fn into(self) -> Vec<String> {
        let mut result_args = Vec::new();
        result_args.extend(self.sources);

        if let Some(java_config) = self.java_config {
            result_args.push("--source".to_string());
            result_args.push(java_config.source.to_string());
            result_args.push("--target".to_string());
            result_args.push(java_config.target.to_string());
        }

        if let Some(output_dir) = self.output_dir {
            result_args.push("-d".to_string());
            result_args.push(output_dir);
        }

        result_args
    }
}

impl JavacConfig {
    pub fn from_sources(sources: Vec<String>) -> JavacConfig {
        Self {
            sources,
            output_dir: None,
            java_config: None
        }
    }

    pub fn new(sources: Vec<String>, output_dir: Option<String>, java_config: Option<JavaConfig>) -> Self {
        Self {
            sources,
            output_dir,
            java_config
        }
    }

    pub fn into_args(self) -> Vec<String> {
        self.into()
    }
}