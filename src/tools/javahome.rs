use std::{collections::HashMap, path::PathBuf};

const JAVA_TOOL_NAME: &'static str = if cfg!(windows) {
    "java.exe"
} else {
    "java"
};

const JAVAC_TOOL_NAME: &'static str = if cfg!(windows) {
    "javac.exe"
} else {
    "javac"
};

const JAR_TOOL_NAME: &'static str = if cfg!(windows) {
    "jar.exe"
} else {
    "jar"
};

pub struct JavaHome {
    java_home: PathBuf,
    java: Option<PathBuf>,
    javac: Option<PathBuf>,
    jar: Option<PathBuf>,
}

impl TryFrom<PathBuf> for JavaHome {
    type Error = std::io::Error;
    fn try_from(home: PathBuf) -> Result<Self, Self::Error> {
        // TODO: Remove code repetition in this function

        let java_path_bin = PathBuf::from(&home).join("bin");
        let java_path = java_path_bin.join(JAVA_TOOL_NAME);
        let java = match std::fs::metadata(&java_path) {
            Ok(f) => {
                if f.is_file() {
                    Some(java_path)
                } else {
                    None
                }
            }
            Err(e) => {
                eprintln!("{e}");
                None
            }
        };

        let jar_path = java_path_bin.join(JAR_TOOL_NAME);
        let jar = match std::fs::metadata(&jar_path) {
            Ok(f) => {
                if f.is_file() {
                    Some(jar_path)
                } else {
                    None
                }
            }
            Err(_) => None
        };

        let javac_path = java_path_bin.join(JAVAC_TOOL_NAME);
        let javac = match std::fs::metadata(&javac_path) {
            Ok(f) => {
                if f.is_file() {
                    Some(javac_path)
                } else {
                    None
                }
            }
            Err(_) => None
        };


        Ok(Self {
            java_home: home,
            java,
            jar,
            javac
        })
    }
}

impl JavaHome {
    pub fn new() -> std::io::Result<Self> {
        let java_home = match crate::tools::get_java_home_path() {
            Some(home_path) => home_path,
            None => return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Couldn't find the $JAVA_HOME dir (not even manually looking for it)."))
        };

        Self::try_from(java_home)
    }

    pub fn get_java(&self) -> &Option<PathBuf> {
        &self.java
    }

    pub fn get_javac(&self) -> &Option<PathBuf> {
        &self.javac
    }

    pub fn get_jar(&self) -> &Option<PathBuf> {
        &self.jar
    }

    /// Checks if all the tools have a registered path.
    /// If at least one of them is `None`, this method will
    /// return `false`
    pub fn is_valid(&self) -> bool {
        self.java.is_some()
            && self.javac.is_some()
            && self.jar.is_some()
    }

    /// This function returns a map containing the name of the tool as a
    /// key, and the `PathBuf` pointing to the tool as a value. If the value
    /// is `None`, it means that it couldn't be found.
    pub fn get_tools(&self) -> HashMap<&'static str, &Option<PathBuf>> {
        let mut hm = HashMap::new();
        hm.insert("javac", &self.javac);
        hm.insert("java", &self.java);
        hm.insert("jar", &self.jar);
        hm
    }

    pub fn get_java_home(&self) -> &PathBuf {
        &self.java_home
    }
}
