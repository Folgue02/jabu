use std::{path::Path, collections::HashMap};

#[derive(Default, PartialEq, Debug, Clone)]
/// Represents a manifest for Jar file, containing a `HashMap<String, String>`
/// made of keys and values.
pub struct JarManifest {
    pub contents: HashMap<String, String>
}

impl std::fmt::Display for JarManifest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n", 
            self.contents.iter()
                .map(|(k, v)| format!("{k}: {v}"))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl From<HashMap<String, String>> for JarManifest {
    fn from(value: HashMap<String, String>) -> Self {
        Self {
            contents: value
        }
    }
}

impl JarManifest {
    /// Creates a `JarManifest` with the given contents.
    pub fn new(contents: HashMap<String, String>) -> Self {
        Self::from(contents)
    }

    /// Writes the contents of the manifest into a file with the format of
    /// a manifest (`k: v`).
    pub fn write_to_file(&self, target: &Path) -> std::io::Result<()> {
        std::fs::write(
            target,
            self.to_string()
        )
    }
}
