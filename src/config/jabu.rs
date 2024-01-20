use std::{path::PathBuf, io::ErrorKind};

use ron::error::SpannedError;
use serde::{Deserialize, Serialize};

use super::java::ProjectType;

/// Represents the configuration of a Java project.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct JabuConfig {
    pub header: ConfigHeader,
    pub java_config: JavaConfig,
    pub fs_schema: FsSchema,
}

impl TryFrom<PathBuf> for JabuConfig {
    type Error = std::io::Error;
    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let file_contents = std::fs::read_to_string(value)?;

        match Self::try_from(file_contents.as_str()) {
            Ok(jabu_config) => Ok(jabu_config),
            Err(e) => {
                Err(std::io::Error::new(ErrorKind::Other, stringify!(e)))
            }
        }
    }
}

impl TryFrom<&str> for JabuConfig {
    type Error = SpannedError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        ron::from_str(value)
    }
}

impl JabuConfig {
    pub fn default_of_name(proj_name: &str, project_type: ProjectType) -> Self {
        Self {
            header: ConfigHeader::of_proj_name(proj_name),
            java_config: JavaConfig::default(),
            fs_schema: FsSchema::new(project_type),
        }
    }
}

/// Header of the project, containing the metadata, such as
/// the name, its author, description etc...
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ConfigHeader {
    pub project_name: String,
    pub author: String,
    pub description: String,
    pub version: String,
}

impl ConfigHeader {
    pub fn of_proj_name(proj_name: &str) -> ConfigHeader {
        Self {
            project_name: proj_name.to_string(),
            author: String::from("anon"),
            description: String::from("A Java project."),
            version: String::from("0.0.1"),
        }
    }
}

/// Configuration about the jdk to be used on the 
/// project.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct JavaConfig {
    /// Minimum version of the jdk to be used
    /// while working on the project.
    pub java_version: u8,
    
    /// Version of the code to be compiled.
    pub source: u8,

    /// Version that will be compatible with the 
    /// produced classes objects.
    pub target: u8,
}

impl Default for JavaConfig {
    fn default() -> Self {
        Self {
            java_version: 17,
            source: 17,
            target: 17,
        }
    }
}

/// Represents the file structure of the project.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct FsSchema {
    pub source: String,
    pub target: String,
    pub resources: String,
    pub test: String,
    pub other: Vec<String>
}

impl FsSchema {
    pub fn new(project_type: ProjectType) -> Self {
        Self {
            source: "./src/main".to_string(),
            target: "./target".to_string(),
            resources: "./src/resources".to_string(),
            test: "./src/test".to_string(),
            other: Vec::new()
        }
    }
}

impl FsSchema {
    pub fn check_integrity(&self, directory: &str) -> Result<(), Vec<String>> {

        Ok(())
    }

    pub fn create(&self, base_directory: &str) -> std::io::Result<()> {
        let mut dirs = Vec::with_capacity(5);
        dirs.push(&self.source);
        dirs.push(&self.target);
        dirs.push(&self.resources);
        dirs.push(&self.test);
        dirs.extend(&self.other);

        for dir in dirs {
            std::fs::create_dir_all(dir)?;
        }

        Ok(())
    }
}
