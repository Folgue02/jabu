use std::path::PathBuf;

/// Represents the different types of files in the 
/// file system.
pub enum FSNodeType {
    /// A regular file.
    File,

    // A regular directory.
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
    /// ```rust
    /// let expected_types = &[FSNodeType::File, FSNodeType::SymLink
    ///
    /// let entry = ...;
    ///
    /// // If this returns `true`, this would mean that the entry points
    /// // either to a file or a symbolic link or both.
    /// FSNodeType::check_match_multiple_types(expected_types, entry);
    /// ```
    fn check_match_multiple_types(types: &[FSNodeType], entry: &walkdir::DirEntry) -> bool {
        if types.is_empty() {
            true
        } else {
            types.iter()
                .any(|node_type| node_type.check_match_type(entry))
        }
    }
}

/// Returns a `Vec` containing all the paths that fullfil the 
/// given predicate.
pub fn walkdir_find<D, F>(directory: D, predicate: F, target_file_types: &[FSNodeType]) -> Vec<PathBuf> 
    where D: AsRef<str>,
          F: Fn(&PathBuf) -> bool {
    walkdir::WalkDir::new(directory.as_ref())
        .into_iter()
        .filter_map(|e| match e {
            Ok(entry) => Some(entry),
            Err(_) => None,
        })
        .filter(|entry| FSNodeType::check_match_multiple_types(target_file_types, &entry))
        .map(|entry| entry.into_path().to_path_buf())
        .collect::<Vec<PathBuf>>()
            /*
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_path_buf())
        .filter(|e| predicate(e))
        .collect::<Vec<PathBuf>>();*/
}
