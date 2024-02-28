use std::{path::{PathBuf, Path}, io::ErrorKind};
use crate::utils::{walkdir_find, FSNodeType};

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
    pub dependencies: DependenciesConfig,
}

impl TryFrom<PathBuf> for JabuConfig {
    type Error = std::io::Error;
    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let file_contents = std::fs::read_to_string(value)?;

        match Self::try_from(file_contents.as_str()) {
            Ok(jabu_config) => Ok(jabu_config),
            Err(e) => {
                Err(std::io::Error::new(ErrorKind::Other, e.to_string()))
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
    /// Generates a project with the given name and project type.
    pub fn default_of_name(proj_name: &str, project_type: ProjectType) -> Self {
        Self {
            header: ConfigHeader::of_proj_name(proj_name),
            java_config: JavaConfig::default(),
            fs_schema: FsSchema::new(project_type),
            properties: HashMap::new(),
            dependencies: DependenciesConfig::default()
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

    /// Returns the path for the generated binaries inside of the target
    /// directory.
    pub fn target_bin(&self) -> String {
        PathBuf::from(&self.target)
            .join("bin")
            .to_string_lossy()
            .to_string()
    }

    pub fn target_docs(&self) -> String {
        PathBuf::from(&self.target)
            .join("docs")
            .to_string_lossy()
            .to_string()
    }

    /// Returns a collection of absolute paths pointing to 
    /// the jar files inside the `lib` directory. This method uses the
    /// [`crate::utils::walkdir_find`] method, which will return an empty 
    /// vector if it couldn't read the specified directory.
    ///
    /// ***NOTE***: This search is recursive, so it will look for jar files
    /// recursively.
    ///
    /// # See
    /// - [`crate::utils::walkdir_find`]
    pub fn get_libs(&self) -> Vec<PathBuf> {
        crate::utils::walkdir_find(
            &self.lib,
            |entry| entry.extension().map_or(false, |ext| ext == "jar"),
            &[FSNodeType::File]
        )
    }

    
    /// Returns a collection of absolute paths pointing to the java sources
    /// in the `sources` directory. This method uses a call to the 
    /// [`crate::utils::walkdir_find`] method, which will return an empty 
    /// vector if it couldn't read the specified directory.
    ///
    /// ***NOTE***: This search is recursive, so it will look for java files
    /// recursively.
    ///
    /// # See
    /// - [`crate::utils::walkdir_find`]
    pub fn get_java_sources(&self) -> Vec<PathBuf> {
        crate::utils::walkdir_find(
            &self.source,
            |entry| entry.extension().unwrap_or_default() == "java",
            &[FSNodeType::File]
        )
    }
}


// TODO: Move all the code below to a different file

/// Specification of a dependency, containing
/// its `artifact_name`, `group_id`, and `version`.
#[derive(PartialEq, Clone, Debug)]
pub struct DependencySpec {
    pub artifact_name: String,
    pub group_id: String,
    pub version: String,
}

impl DependencySpec {
    #[allow(unused)]
    pub fn new<T>(artifact_name: T, group_id: T, version: T) -> Self 
        where T: Into<String>{
        Self {
            artifact_name: artifact_name.into(),
            group_id: group_id.into(),
            version: version.into()
        }
    }
}

impl Serialize for DependencySpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(format!("{}-{}-{}", self.group_id, self.artifact_name, self.version).as_str())
    }
}

impl TryFrom<&str> for DependencySpec {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let splitted = value.splitn(3, '-').collect::<Vec<&str>>();

        if splitted.len() < 3 {
            Err(())
        } else {
            Ok(
                DependencySpec {
                    artifact_name: splitted[0].to_string(),
                    group_id: splitted[1].to_string(),
                    version: splitted[2].to_string()
                }
            )
        }
    }
}

impl Into<String> for DependencySpec {
    fn into(self) -> String {
        format!("{}-{}-{}", self.group_id, self.artifact_name, self.version)
    }
}

impl<'de> Deserialize<'de> for DependencySpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        let spec_string: String = Deserialize::deserialize(deserializer)?;
        match DependencySpec::try_from(spec_string.as_str()) {
            Ok(spec) => Ok(spec),
            Err(_) => Err(serde::de::Error::custom("Couldn't parse dependency specification"))
        }
    }
}

/// Represents the configuration of the project's dependencies.
#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Debug)]
pub struct DependenciesConfig {
    /// List of local dependencies. Jabu will check that these exist in the `lib` dir
    /// with their corresponding name + .jar
    pub local: Vec<DependencySpec>,

    /// A map, representing the remote dependencies. **The key represents the
    /// URL of the repository from where to fetch from>**, and the value, being a list
    /// are the dependencies to fetch.
    pub remote: HashMap<String, Vec<DependencySpec>>
}
