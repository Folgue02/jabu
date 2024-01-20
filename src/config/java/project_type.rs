#[derive(Debug, PartialEq, Eq)]
/// Represents the type of Jabu projects for Java.
pub enum ProjectType {
    /// Project with the purpose of generating an executable output.
    Binary
}

impl TryFrom<&str> for ProjectType {
    type Error = ();
    fn try_from(value: &str) -> Result<ProjectType, Self::Error> {
        match value.to_lowercase().as_str() {
            "binary" | "executable" | "bin" => Ok(Self::Binary),
            _ => Err(())
        }
    }
}
