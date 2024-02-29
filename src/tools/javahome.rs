use std::{collections::HashMap, path::PathBuf};
use prettytable::{Row, Attr, Cell, color};

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

const JAVADOC_TOOL_NAME: &'static str = if cfg!(windows) {
    "javadoc.exe"
} else {
    "javadoc"
};

const JPACKAGE_TOOL_NAME: &'static str = if cfg!(windows) {
    "jpackage.exe"
} else {
    "jpackage"
};

/// Structure that holds the paths to the different tools provided 
/// by the jdk. This structure might not contain the paths for
/// all the existing tools, or not even for one tool.
///
/// # See
/// - [`JavaHome::get_java`], as an example for retrieving the path of a tool.
/// - [`JavaHome::is_valid`], which checks if **all** tools are available.
pub struct JavaHome {
    java_home: PathBuf,
    java: Option<PathBuf>,
    javac: Option<PathBuf>,
    jar: Option<PathBuf>,
    javadoc: Option<PathBuf>,
    jpackage: Option<PathBuf>
}

/// Checks if the given path points to a file, if it exists, it
/// will be returned as `Some(T)`, if not, `None` is returned.
fn if_path_exists(path: PathBuf) -> Option<PathBuf> {
    std::fs::metadata(&path)
        .ok()
        .filter(|md| md.is_file())
        .map(|_| path)
}

impl TryFrom<PathBuf> for JavaHome {
    type Error = std::io::Error;
    fn try_from(home: PathBuf) -> Result<Self, Self::Error> {
        let java_path_bin = PathBuf::from(&home).join("bin");
let java_path = java_path_bin.join(JAVA_TOOL_NAME);
        let java = if_path_exists(java_path);

        let jar_path = java_path_bin.join(JAR_TOOL_NAME);
        let jar = if_path_exists(jar_path);

        let javadoc_path = java_path_bin.join(JAVADOC_TOOL_NAME);
        let javadoc = if_path_exists(javadoc_path); 

        let javac_path = java_path_bin.join(JAVAC_TOOL_NAME);
        let javac = if_path_exists(javac_path); 

        let jpackage_path = java_path_bin.join(JPACKAGE_TOOL_NAME);
        let jpackage = if_path_exists(jpackage_path);

        Ok(Self {
            java_home: home,
            java,
            jar,
            javac,
            javadoc,
            jpackage
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

    /// Path to the 'java' tool.
    pub fn get_java(&self) -> &Option<PathBuf> {
        &self.java
    }

    /// Path to the 'javac' tool.
    pub fn get_javac(&self) -> &Option<PathBuf> {
        &self.javac
    }

    /// Path to the 'jar' tool.
    pub fn get_jar(&self) -> &Option<PathBuf> {
        &self.jar
    }

    /// Path to the 'javadoc' tool.
    pub fn get_javadoc(&self) -> &Option<PathBuf> {
        &self.javadoc
    }

    /// Path to the 'javadoc' tool.
    pub fn get_jpackage(&self) -> &Option<PathBuf> {
        &self.jpackage
    }

    /// Checks if all the tools have a registered path.
    /// If at least one of them is `None`, this method will
    /// return `false`.
    pub fn is_valid(&self) -> bool {
        self.java.is_some()
            && self.javac.is_some()
            && self.jar.is_some()
            && self.javadoc.is_some()
    }

    /// Given a list of tool names, map them into a hashmap that contains the
    /// name of the tool, pointing to a boolean that tells if the tool is available 
    /// or not.
    ///
    /// # Examples
    /// ```rust
    /// // The 'java' tool is available, but 'javac' isn't.
    /// let mut expected = HashMap::new();
    /// expected.insert("java", true);
    /// expected.insert("javac", false);
    ///
    /// assert_eq!(expected, java_home.check_required_tools(vec!["java", "javac"]))
    /// ```
    ///
    /// This method is usually used along the [`crate::tasks::JabuTask::required_tools`] method.
    ///
    /// ```
    /// let task: Task = ...;
    /// java_home.check_required_tools(task.required_tools());
    /// ```
    pub fn check_required_tools(&self, required_tools: &[&'static str]) -> HashMap<&'static str, bool> {
        let tools_mp = self.get_tools();
        required_tools.iter()
            .map(|tool_name| (*tool_name, tools_mp.contains_key(tool_name)))
            .collect()
    }

    /// This function returns a map containing the name of the tool as a
    /// key, and the `PathBuf` pointing to the tool as a value. If the value
    /// is `None`, it means that it couldn't be found.
    pub fn get_tools(&self) -> HashMap<&'static str, &Option<PathBuf>> {
        let mut hm = HashMap::new();
        hm.insert("javac", &self.javac);
        hm.insert("java", &self.java);
        hm.insert("jar", &self.jar);
        hm.insert("javadoc", &self.javadoc);
        hm.insert("jpackage", &self.jpackage);
        hm
    }

    /// Path pointing to the java home.
    pub fn get_java_home(&self) -> &PathBuf {
        &self.java_home
    }

    /// Checks if the tool for the corresponding name was
    /// detected. If `false` is returned, this would mean that the
    /// specified tool is not available.
    pub fn has_tool(&self, tool_name: &str) -> bool {
        self.get_tools().contains_key(tool_name)
    }

    /// Generates a table containing two columns, one with the name 
    /// of the tool, and the other displaying if the tool is available/found
    /// or not.
    ///
    /// # Example
    /// ```rust
    /// let java_home = JavaHome::new()?;
    /// 
    /// println!("{}", java_home.print_tool_availability_table();
    /// // |Tool name | Availability |
    /// // |----------|--------------|
    /// // | java     | Available    |
    /// // | javac    | Not available|
    /// // ...
    /// ```
    pub fn print_tool_availability_table(&self) {
        let mut table = prettytable::Table::new();

        table.set_format(*prettytable::format::consts::FORMAT_BORDERS_ONLY);
        table.set_titles(
            Row::new(
                vec![
                    Cell::new("Tool name"),
                    Cell::new("Availability")
                ]
            )
        );

        self.get_tools().iter()
            .map(|(tool_name, path)| {
                Row::new(
                    vec![
                        Cell::new(tool_name)
                            .with_style(Attr::ForegroundColor(color::BLUE)),
                        if path.is_some() {
                            Cell::new("Available")
                                .with_style(Attr::ForegroundColor(color::GREEN))
                                .with_style(Attr::Bold)
                        } else {
                            Cell::new("Not available")
                                .with_style(Attr::ForegroundColor(color::RED))
                                .with_style(Attr::Bold)
                        }
                    ]
                )
            })
            .for_each(|row| {table.add_row(row);});

        table.printstd();
    }
}
