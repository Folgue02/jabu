use std::{collections::HashMap, path::Path};
use serde::{Serialize, Deserialize};

/// Represents the type of Jabu projects for Java.
#[derive(Debug, PartialEq, Eq)]
pub enum ProjectType {
    /// Project with the purpose of generating an executable output.
    Binary,
}

impl TryFrom<&str> for ProjectType {
    type Error = ();
    fn try_from(value: &str) -> Result<ProjectType, Self::Error> {
        match value.to_lowercase().as_str() {
            "binary" | "executable" | "bin" => Ok(Self::Binary),
            _ => Err(()),
        }
    }
}

/// Represents a manifest for Jar file, containing a `HashMap<String, String>`
/// made of keys and values.
#[derive(Default, PartialEq, Debug, Clone, Deserialize, Serialize)]
pub struct JarManifest {
    pub contents: HashMap<String, String>,
}

impl JarManifest {
    pub fn write_to_file(&self, file_path: impl AsRef<Path>) -> std::io::Result<()> {
        std::fs::write(file_path, self.to_string())
    }

    pub fn get(&self, key: impl AsRef<str>) -> Option<&String> {
        self.contents.get(key.as_ref())
    }
}

impl ToString for JarManifest {
    fn to_string(&self) -> String {
        self.contents
            .iter()
            .map(|(k, v)| format!("{k}: {v}"))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl From<HashMap<String, String>> for JarManifest {
    fn from(value: HashMap<String, String>) -> Self {
        Self { contents: value }
    }
}

impl Into<HashMap<String, String>> for JarManifest {
    fn into(self) -> HashMap<String, String> {
        self.contents
    }
}

impl AsRef<HashMap<String, String>> for JarManifest {
    fn as_ref(&self) -> &HashMap<String, String> {
        &self.contents
    }
}
