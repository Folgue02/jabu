use jabu_config::model::ArtifactSpec;

/// An error while performing an operation on the local
/// repository. This can also include operations such fetching a
/// remote artifact and saving it to the local repository.
#[derive(Debug)]
pub enum RepositoryOperationError {
    /// An error caused when interacting with the local
    /// repository.
    IoError(std::io::Error),

    /// An error caused when trying to access a an artifact
    /// that its not registered in the repository.
    ArtifactNotFound(ArtifactSpec)
}

impl std::fmt::Display for RepositoryOperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self) // TODO
    }
}

impl From<std::io::Error> for RepositoryOperationError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl std::error::Error for RepositoryOperationError {}
