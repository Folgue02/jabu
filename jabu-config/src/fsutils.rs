use crate::model::{JabuProject, JABU_FILE_NAME};
use ron::error::SpannedError;
use std::{
    fs::{create_dir_all, read_to_string, write},
    path::{Path, PathBuf},
};

/// Represents an error while loading a project from the
/// filesystem.
#[derive(Debug)]
pub enum ProjectLoadingError {
    /// Error caused by an IO operation.
    IoError(std::io::Error),

    /// Error caused when parsing a malformed project.
    FileParsingError(SpannedError),
}

impl From<std::io::Error> for ProjectLoadingError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<SpannedError> for ProjectLoadingError {
    fn from(value: SpannedError) -> Self {
        Self::FileParsingError(value)
    }
}

impl std::fmt::Display for ProjectLoadingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Self::IoError(e) => {
                format!("Project loading error of type io: {e}")
            }
            Self::FileParsingError(e) => {
                format!("Project loading error when parsing the contents: {e}")
            }
        };
        write!(f, "{string}")
    }
}

impl std::error::Error for ProjectLoadingError {}

fn dir_or_cwd(base_directory: Option<PathBuf>) -> PathBuf {
    if let Some(bd) = base_directory {
        bd.into()
    } else {
        std::env::current_dir().unwrap()
    }
}

/// Returns a vector containing all the directories of the given
/// jabu project. This doesn't take into account those generated with the
/// [`crate::model::FsSchema::generated_files`].
pub fn project_dirs(jabu_project: &JabuProject) -> Vec<PathBuf> {
    let project_dir = PathBuf::from(&jabu_project.header.project_name);
    let mut dirs = vec![
        // Base directory of the project.
        project_dir.clone(),
        // Source directories.
        project_dir.clone().join(&jabu_project.fs_schema.source),
        project_dir.clone().join(&jabu_project.fs_schema.resources),
        project_dir
            .clone()
            .join(&jabu_project.fs_schema.source)
            .join("main"),
        project_dir.clone().join(&jabu_project.fs_schema.target),
        project_dir.clone().join(&jabu_project.fs_schema.lib),
        project_dir.clone().join(&jabu_project.fs_schema.scripts),
    ];
    // Other directories
    dirs.extend(
        jabu_project
            .fs_schema
            .other
            .iter()
            .map(|d| project_dir.clone().join(d))
            .collect::<Vec<PathBuf>>(),
    );
    dirs
}

/// Creates the file structure of the given jabu project (*i.e; the sources directory,
/// the scripts directory...*)
///
/// # Note
///
/// This function also creates the files specified in [`crate::model::FsSchema::generated_files`]
pub fn create_project(
    base_directory: Option<PathBuf>,
    jabu_project: &JabuProject,
) -> std::io::Result<()> {
    let base_dir = dir_or_cwd(base_directory);
    let project_dir = base_dir.join(&jabu_project.header.project_name);
    let dirs = project_dirs(jabu_project);

    // Create all directories
    dirs.into_iter().try_for_each(|dir| create_dir_all(dir))?;

    jabu_project
        .fs_schema
        .generated_files
        .iter()
        .try_for_each(|(filename, contents)| {
            let filename = project_dir.clone().join(filename);
            create_dir_all(filename.parent().unwrap_or(Path::new(".")))?;
            write(filename, contents)
        })
}

/// Loads a project from a given path to a directory. This function calls
/// the [`project_from_file`] function by passing it
/// `base_directory.join(crate::model::JABU_FILE_NAME)`.
///
/// # See
/// * [`project_from_file`]
pub fn project_from_directory(
    base_directory: Option<PathBuf>,
) -> Result<JabuProject, ProjectLoadingError> {
    let base_directory = dir_or_cwd(base_directory);
    project_from_file(base_directory.join(JABU_FILE_NAME))
}

/// Loads the project from a file.
pub fn project_from_file(filepath: PathBuf) -> Result<JabuProject, ProjectLoadingError> {
    Ok(ron::from_str(&read_to_string(filepath)?)?)
}

/// Returns the paths to all .java files in the `sources` directory of the project.
pub fn java_sources(
    base_directory: Option<PathBuf>,
    jabu_project: &JabuProject,
) -> Vec<PathBuf> {
    walkdir_find(
        dir_or_cwd(base_directory).join(&jabu_project.fs_schema.source),
        |file_name| file_name.extension().unwrap_or_default() == "java",
        &[FSNodeType::File],
    )
}

/// Returns the paths to all .jar files in the `lib` directory of the project.
pub fn libs(base_directory: Option<PathBuf>, jabu_project: &JabuProject) -> Vec<PathBuf> {
    walkdir_find(
        dir_or_cwd(base_directory).join(&jabu_project.fs_schema.lib),
        |file_name| file_name.extension().unwrap_or_default() == "jar",
        &[FSNodeType::File],
    )
}

/// Represents the different types of files in the
/// file system.
pub enum FSNodeType {
    /// A regular file.
    File,

    /// A regular directory.
    Dir,

    /// A symlink to either a file or a directory.
    SymLink,
}

impl FSNodeType {
    /// Checks if the given `walkdir::DirEntry` is of the same type
    /// as the `FSNodeType` variant.
    fn check_match_type(&self, entry: &walkdir::DirEntry) -> bool {
        match self {
            Self::Dir => entry.file_type().is_dir(),
            Self::File => entry.file_type().is_file(),
            Self::SymLink => entry.file_type().is_symlink(),
        }
    }

    /// Checks if the given `entry` matches some the given `types`. This
    /// given types might be exclusive against each other, such as `FSNodeType::Dir` and
    /// `FSNodeType::File` for example, it's up to the user to make sure that the given
    /// predicative types make sense.
    ///
    /// ## NOTE
    /// If `types` is empty, this function will always return `true`, it's assumed that
    /// if there is no predicate, then everything is valid.
    ///
    /// # Example
    /// ```not_rust
    /// let expected_types = &[FSNodeType::File, FSNodeType::SymLink];
    ///
    /// let entry = todo!(); 
    ///
    /// // If this returns `true`, this would mean that the entry points
    /// // either to a file or a symbolic link or both.
    /// Self::check_match_multiple_types(expected_types, entry);
    /// ```
    fn check_match_multiple_types(types: &[Self], entry: &walkdir::DirEntry) -> bool {
        if types.is_empty() {
            true
        } else {
            types
                .iter()
                .any(|node_type| node_type.check_match_type(entry))
        }
    }
}

/// Returns a `Vec` containing all the paths that fullfil the
/// given predicate.
pub fn walkdir_find<D, F>(
    directory: D,
    predicate: F,
    target_file_types: &[FSNodeType],
) -> Vec<PathBuf>
where
    D: Into<PathBuf>,
    F: Fn(&PathBuf) -> bool,
{
    walkdir::WalkDir::new(directory.into())
        .into_iter()
        .filter_map(|e| match e {
            Ok(entry) => Some(entry),
            Err(_) => None,
        })
        .filter(|entry| FSNodeType::check_match_multiple_types(target_file_types, &entry))
        .map(|entry| entry.into_path().to_path_buf())
        .filter(|entry_path| predicate(entry_path))
        .collect::<Vec<PathBuf>>()
    /*
    .filter(|e| e.file_type().is_file())
    .map(|e| e.path().to_path_buf())
    .filter(|e| predicate(e))
    .collect::<Vec<PathBuf>>();*/
}
