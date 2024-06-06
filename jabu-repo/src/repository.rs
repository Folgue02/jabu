use crate::RepositoryOperationResult;
use jabu_config::model::ArtifactSpec;
use std::{
    collections::HashSet,
    fs::{read_dir, File},
    io::copy,
    path::{Path, PathBuf},
};

/// Represents a local repository. This structure can be used
/// for managing the local repository, creating, reading and
/// deleting artifacts.
#[derive(Clone, PartialEq, Debug)]
pub struct Repository {
    pub base_path: PathBuf,
}

impl Default for Repository {
    fn default() -> Self {
        let home_directory = if cfg!(windows) {
            std::env::var("USERPROFILE").unwrap_or_default()
        } else {
            std::env::var("HOME").unwrap_or_default()
        };
        Self {
            base_path: PathBuf::from(home_directory).join("./jaburepo"),
        }
    }
}

impl Repository {
    pub fn new<T: Into<PathBuf>>(base_path: T) -> Self {
        Self {
            base_path: base_path.into(),
        }
    }

    fn artifact_as_dirname(&self, artifact: &ArtifactSpec) -> PathBuf {
        self.base_path
            .join(&artifact.author)
            .join(&artifact.artifact_id)
    }

    /// Returns the path to the jar of the given artifact
    ///
    /// # Note
    /// This method doesn't check if the given artifact exists in the
    /// repository, it only formats the path to the jar.
    pub fn jar_path(&self, artifact: &ArtifactSpec) -> PathBuf {
        let mut path = self.artifact_as_dirname(artifact).join(&artifact.version);
        path.set_extension(
            path.extension()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
                + ".jar",
        );
        path
    }

    /// Returns the path to the jabu.ron file of the given artifact
    ///
    /// # Note
    /// This method doesn't check if the given artifact exists in the
    /// repository, it only formats the path to the pom.
    pub fn jaburon_path(&self, artifact: &ArtifactSpec) -> PathBuf {
        let mut path = self.artifact_as_dirname(artifact).join(&artifact.version);
        path.set_extension(
            path.extension()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
                + ".ron",
        );
        path
    }

    /// Checks if the user exists in the repo.
    pub fn author_exists(&self, author: impl AsRef<str>) -> bool {
        self.base_path.join(author.as_ref()).exists()
    }

    /// Checks if there is an artifact registered by the author given.
    ///
    /// # NOTE
    /// `false` is both returned if the author doesn't exists or he does but the
    /// artifact doesn't (*there is no distinction*).
    pub fn artifact_exists<A: AsRef<str>, B: AsRef<str>>(
        &self,
        author: A,
        artifact_name: B,
    ) -> bool {
        self.base_path
            .join(author.as_ref())
            .join(artifact_name.as_ref())
            .exists()
    }

    /// Checks if the artifact exists.
    ///
    /// # NOTE
    /// This method is a shortcut to manually checking if the artifact's ron file exists.
    /// ```not_rust
    /// jaburon_path(artifact).exists()
    /// ```
    pub fn exists(&self, artifact: &ArtifactSpec) -> bool {
        self.jaburon_path(artifact).exists()
    }

    pub fn save_artifact<T: AsRef<[u8]>>(
        &self,
        artifact: &ArtifactSpec,
        artifact_content: T,
        jaburon_content: T,
    ) -> std::io::Result<()> {
        self.save_artifact_standalone(artifact, artifact_content)?;
        self.save_jaburon_standalone(artifact, jaburon_content)?;
        Ok(())
    }

    /// Writes the jar's content to its correspondent file in the repository.
    ///
    /// Sample location of an artifact's jar: `group_id/artifact_id/version.xml`
    fn save_artifact_standalone<T: AsRef<[u8]>>(
        &self,
        artifact: &ArtifactSpec,
        artifact_content: T,
    ) -> std::io::Result<PathBuf> {
        let artifact_dirname = self.artifact_as_dirname(artifact);
        let jar_path = self.jar_path(artifact);

        std::fs::create_dir_all(artifact_dirname)?;
        copy(
            &mut artifact_content.as_ref(),
            &mut File::create(&jar_path)?,
        )?;
        Ok(jar_path)
    }

    /// Writes the jaburon content to its correspondent file in the repository.
    ///
    /// Sample location of a xml: `group_id/artifact_id/version.ron`
    fn save_jaburon_standalone<T: AsRef<[u8]>>(
        &self,
        artifact: &ArtifactSpec,
        artifact_content: T,
    ) -> std::io::Result<PathBuf> {
        let artifact_dirname = self.artifact_as_dirname(artifact);
        let jaburon_path = self.jaburon_path(artifact);

        std::fs::create_dir_all(artifact_dirname)?;
        copy(
            &mut artifact_content.as_ref(),
            &mut File::create(&jaburon_path)?,
        )?;
        Ok(jaburon_path)
    }

    /// Returns an immutable reference to the path where the repository is located at.
    pub fn base_path(&self) -> &PathBuf {
        &self.base_path
    }

    /// Returns a vector containing all the names of artifacts of the given author. If the author
    /// doesn't exist, `None` is returned.
    pub fn get_author_artifacts(&self, author_name: impl AsRef<str>) -> Option<Vec<String>> {
        if self.author_exists(author_name.as_ref()) {
            Some(
                std::fs::read_dir(self.base_path().join(author_name.as_ref()))
                    .unwrap()
                    .into_iter()
                    .filter_map(|entry| entry.ok())
                    .filter(|entry| {
                        if let Ok(entry_metadata) = entry.metadata() {
                            !entry_metadata.is_file()
                        } else {
                            false
                        }
                    })
                    .map(|entry| entry.file_name().to_string_lossy().to_string())
                    .collect(),
            )
        } else {
            None
        }
    }

    /// Returns a collection of strings representing the versions of the specified artifact. If the
    /// artifact doesn't exist, this method will return `None` instead of an empty vector.
    ///
    /// # Note
    /// **As long as the repository's integrity is fine**, this method *should* never return an empty
    /// vector.
    pub fn get_artifact_versions(
        &self,
        author_name: impl AsRef<str>,
        artifact_name: impl AsRef<str>
    ) -> Option<Vec<String>> {
        // Creates a an artifact with a random version (it gets ignored)
        let artifact = ArtifactSpec::new(author_name.as_ref(), artifact_name.as_ref(), "0.0.0" );
        dbg!(&self.artifact_as_dirname(&artifact));
        Some(
            read_dir(self.artifact_as_dirname(&artifact))
                .ok()?
                .into_iter()
                .filter_map(|element| element.ok())
                .filter(|element| element.path().extension().unwrap_or_default() == "ron")
                .map(|element| {
                    element
                        .path()
                        .file_stem()
                        .unwrap()
                        .to_string_lossy()
                        .to_string()
                })
                .collect::<Vec<String>>(),
        )
    }
}
