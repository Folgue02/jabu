use crate::config::JavaConfig;

/// Represents the visibility of elements in java,
/// such as `public`, `protected` or `private`.
#[derive(Debug, PartialEq, Clone)]
pub enum JavaVisibilityLevel {
    Private,
    Protected,
    Public
}

impl Into<&'static str> for JavaVisibilityLevel {
    fn into(self) -> &'static str {
        match self {
            Self::Private => "private",
            Self::Protected => "protected",
            Self::Public => "public",
        }
    }
}

impl Into<String> for JavaVisibilityLevel {
    fn into(self) -> String {
        let result: &'static str = self.into();
        result.to_string()
    }
}

impl TryFrom<&str> for JavaVisibilityLevel {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.to_lowercase();
        match value.as_str() {
            "private" => Ok(Self::Private),
            "public" => Ok(Self::Public),
            "protected" => Ok(Self::Protected),
            _ => Err(())
        }
    }
}

/// API for generating configurations for executing 
/// the `javadoc` cli utility.
#[derive(Debug, PartialEq)]
pub struct JavadocToolConfig {
    sources: Vec<String>,
    output_dir: Option<String>,
    java_config: Option<JavaConfig>,
    visibility_level: JavaVisibilityLevel,
    pub classpath: Vec<String>,
}

impl Into<Vec<String>> for JavadocToolConfig {
    fn into(self) -> Vec<String> {
        let mut result = Vec::new();

        result.extend(self.sources);

        if let Some(output_dir) = self.output_dir {
            result.push("-d".to_string());
            result.push(output_dir)
        }
        
        if let Some(java_config) = self.java_config {
            result.push("--source".to_string());
            result.push(java_config.source.to_string());
        }

        result.push(format!("-{}", <JavaVisibilityLevel as Into<String>>::into(self.visibility_level)));

        result
    }
}

impl JavadocToolConfig {
    pub fn new(sources: Vec<String>,
               output_dir: Option<String>,
               java_config: Option<JavaConfig>,
               visibility_level: JavaVisibilityLevel
           ) -> Self {
        Self {
            java_config,
            output_dir,
            sources,
            classpath: Vec::new(),
            visibility_level
        }
    }

    /// Generates the arguments for the `javadoc` 
    /// cli tool, consuming itself.
    pub fn into_args(self) -> Vec<String> {
        self.into()
    }
}

