use std::{path::{PathBuf, Path}, io::ErrorKind};

use ron::error::SpannedError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::java::ProjectType;

pub const JABU_FILE_NAME: &'static str = "jabu.ron";

/// Represents the configuration of a Java project.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct JabuConfig {
    pub header: ConfigHeader,
    pub java_config: JavaConfig,
    pub fs_schema: FsSchema,
    pub properties: HashMap<String, String>,
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
            properties: HashMap::new()
        }
    }

    /// Returns the display name of the project. Serves as a standard
    /// for representing a project. (*`{project name}-{project version}`*)
    pub fn display_name(&self) -> String {
        format!("{}-{}", self.header.project_name, self.header.version)
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
    /// Creates a configuration header containing the given name
    /// and the default values for the rest of the fields.
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
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
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
    pub lib: String,
    pub resources: String,
    pub scripts: String,
    pub test: String,
    pub other: Vec<String>
}

impl FsSchema {
    pub fn new(project_type: ProjectType) -> Self {
        Self {
            source: "./src/main".to_string(),
            target: "./target".to_string(),
            lib: "./lib".to_string(),
            resources: "./src/resources".to_string(),
            scripts: "./scripts/".to_string(),
            test: "./src/test".to_string(),
            other: Vec::new()
        }
    }
}

impl FsSchema {
    pub fn create(&self, base_directory: &str) -> std::io::Result<()> {
        let mut dirs = Vec::with_capacity(7);
        dirs.push(&self.source);
        dirs.push(&self.target);
        dirs.push(&self.lib);
        dirs.push(&self.resources);
        dirs.push(&self.test);
        dirs.push(&self.scripts);
        dirs.extend(&self.other);

        for dir in dirs {
            let joined_dirs = Path::new(base_directory).join(dir).to_string_lossy().to_string();
            std::fs::create_dir_all(joined_dirs)?;
        }

        Ok(())
    }

    /// Returns the path for the generated classes inside of the target dir.
    pub fn target_classes(&self) -> String {
        PathBuf::from(&self.target)
            .join("classes")
            .to_string_lossy()
            .to_string()
    }

    /// Returns the path for the generated binaries inside of the target dir.
    pub fn target_bin(&self) -> String {
        PathBuf::from(&self.target)
            .join("bin")
            .to_string_lossy()
            .to_string()
    }
}
