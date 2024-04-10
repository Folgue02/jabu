use super::{JarManifest, ProjectType};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

pub const JABU_FILE_NAME: &'static str = "jabu.ron";

/// Represents the configuration of a Java project.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct JabuProject {
    pub header: ConfigHeader,
    pub java_config: JavaConfig,
    pub manifest: JarManifest,
    pub fs_schema: FsSchema,
    pub dependencies: DependenciesConfig,
}

impl JabuProject {
    pub fn display_name(&self) -> String {
        format!(
            "{}:{}:{}",
            self.header.author, self.header.project_name, self.header.version
        )
    }

    pub fn default_of_name(project_name: impl Into<String>, project_type: ProjectType) -> Self {
        let project_name = project_name.into();
        let mut manifest = HashMap::new();
        manifest.insert("Main-Class".to_string(), "App".to_string());
        Self {
            header: ConfigHeader::of_proj_name(&project_name),
            manifest: JarManifest::from(manifest),
            fs_schema: FsSchema::new(project_type),
            java_config: JavaConfig::default(),
            dependencies: DependenciesConfig::default()
        }
    }
}

/// Header of the project, containing the metadata, such as
/// the name, its author, description etc...
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
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
    /// produced class objects.
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
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct FsSchema {
    /// Directory containing all the source files of the project.
    pub source: String,

    /// Directory containing all the generated files of the project.
    pub target: String,

    /// Directory containing the dependencies of the project.
    pub lib: String,

    /// The resources directory of the project.
    pub resources: String,

    /// Directory where all the scripts are stored.
    pub scripts: String,

    /// Directory containing the tests of the project.
    pub test: String,

    /// The files to generate during the project creation.
    ///
    /// This is a map containing the filename as a key, and the
    /// contents of the file as a value.
    ///
    /// Due to the annotation `#[serde(skip)]` given, this is not stored nor fetched
    /// from the jabu.ron file.
    #[serde(skip)]
    pub generated_files: HashMap<&'static str, &'static str>,

    /// Other directories to create.
    pub other: Vec<String>,
}

impl FsSchema {
    pub fn new(_: ProjectType) -> Self {
        // TODO: Use the ProjectType to generate different projects
        // depending on it.
        let mut generated_files = HashMap::new();
        generated_files.insert(
            "./src/main/App.java",
            r#"
/*
 * Auto-generated file by Jabu.
 */

public class App {
    public static void main(String[] args) {
        System.out.println("Hello World from Jabu!");
    }
}"#,
        );
        Self {
            source: "./src/main".to_string(),
            target: "./target".to_string(),
            lib: "./lib".to_string(),
            resources: "./src/resources".to_string(),
            scripts: "./scripts/".to_string(),
            test: "./src/test".to_string(),
            generated_files,
            other: Vec::new(),
        }
    }

    /// Returns the path for the generated classes inside of the target dir.
    pub fn target_classes(&self) -> PathBuf {
        PathBuf::from(&self.target).join("classes")
    }

    /// Returns the path for the generated binaries inside of the target
    /// directory.
    pub fn target_bin(&self) -> PathBuf {
        PathBuf::from(&self.target).join("bin")
    }

    /// Returns the path for the generated javadoc inside of the target
    /// directory.
    pub fn target_docs(&self) -> PathBuf {
        PathBuf::from(&self.target).join("docs")
    }

    /// Returns the path for the generated self-contained applications
    /// inside of the target directory.
    pub fn target_self_contained(&self) -> PathBuf {
        PathBuf::from(&self.target).join("self-contained")
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArtifactSpec {
    pub author: String,
    pub artifact_id: String,
    pub version: String,
}

/// Represents the configuration of the project's dependencies.
#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Debug)]
pub struct DependenciesConfig {
    /// List of local dependencies. Jabu will check that these exist in the `lib` dir
    /// with their corresponding name + .jar
    pub local: Vec<ArtifactSpec>,

    /// A map, representing the remote dependencies. **The key represents the
    /// URL of the repository from where to fetch from**, and the value, being a list
    /// are the dependencies to fetch.
    pub remote: Vec<ArtifactSpec>,
}

impl<'de> Deserialize<'de> for ArtifactSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let spec_string: String = Deserialize::deserialize(deserializer)?;
        match ArtifactSpec::try_from(spec_string.as_str()) {
            Ok(spec) => Ok(spec),
            Err(_) => Err(serde::de::Error::custom(
                "Couldn't parse dependency specification",
            )),
        }
    }
}

impl Into<String> for ArtifactSpec {
    fn into(self) -> String {
        format!("{}:{}:{}", self.author, self.artifact_id, self.version)
    }
}

impl TryFrom<&str> for ArtifactSpec {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let splitted = value.splitn(3, ':').collect::<Vec<&str>>();

        if splitted.len() < 3 {
            Err(())
        } else {
            Ok(ArtifactSpec {
                author: splitted[0].to_string(),
                artifact_id: splitted[1].to_string(),
                version: splitted[2].to_string(),
            })
        }
    }
}

impl Serialize for ArtifactSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(
            format!("{}:{}:{}", self.author, self.artifact_id, self.version).as_str(),
        )
    }
}

impl ArtifactSpec {
    #[allow(unused)]
    pub fn new<T>(author: T, artifact_id: T, version: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            artifact_id: artifact_id.into(),
            author: author.into(),
            version: version.into(),
        }
    }
}
