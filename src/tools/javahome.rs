use std::path::PathBuf;

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

        let java_path = home.join("java");
        let java = match std::fs::metadata(&java_path) {
            Ok(f) => {
                if f.is_file() {
                    Some(java_path)
                } else {
                    None
                }
            }
            Err(_) => None
        };

        let jar_path = home.join("jar");
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

        let javac_path = home.join("javac");
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

    pub fn is_valid(&self) -> bool {
        self.java.is_some()
            && self.javac.is_some()
            && self.jar.is_some()
    }
}