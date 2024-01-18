use serde::{Deserialize, Serialize};

/// Represents the configuration of a Java project.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct JabuConfig {
    pub header: ConfigHeader,
    pub java_config: JavaConfig,
    pub fs_schema: FsSchema,
}

impl JabuConfig {
    pub fn default_of_name(proj_name: &str) -> Self {
        Self {
            header: ConfigHeader::of_proj_name(proj_name),
            java_config: JavaConfig::default(),
            fs_schema: FsSchema::default(),
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
}

impl Default for FsSchema {
    fn default() -> Self { 
        Self {
            source: "./src/main".to_string(),
            target: "./target".to_string(),
            resources: "./src/resources".to_string(),
            test: "./src/test".to_string(),
        }
    }
}
