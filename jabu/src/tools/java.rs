/// Represents the type of execution of the `java` command.
pub enum JavaExecTarget{
    /// Execute an specified class.
    MainClass(String),

    /// Execute an specified jar.
    Jar(String),
}

pub struct JavaToolConfig {
    java_exec_target: JavaExecTarget,
    pub classpath: Vec<String>,
    arguments: Vec<String>
}

impl Into<Vec<String>> for JavaToolConfig {
    fn into(self) -> Vec<String> {
        let mut args = Vec::new();

        if !self.classpath.is_empty() {
            args.push("-cp".to_string());
            let delimiter = if cfg!(windows) {
                ";"
            } else {
                ":"
            };
            args.push(self.classpath.join(delimiter));
        }

        match self.java_exec_target {
            JavaExecTarget::Jar(jar_name) => {
                args.push("-jar".to_string());
                args.push(jar_name.to_string());
            }
            JavaExecTarget::MainClass(main_class) => {
                args.push(main_class.to_string());
            }
        }

        args.extend(self.arguments);

        args
    }
}

impl JavaToolConfig {
    pub fn new(java_exec_target: JavaExecTarget, classpath: Vec<String>, arguments: Vec<String>) -> Self {
        Self {
            java_exec_target,
            classpath,
            arguments,
        }
    }

    pub fn into_args(self) -> Vec<String> {
        self.into()
    }
}
