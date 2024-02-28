mod java;
mod jar;
mod javac;
mod javahome;
mod javadoc;

use std::path::PathBuf;

pub use java::*;
pub use javadoc::*;
pub use jar::*;
pub use javac::*;
pub use javahome::*;


/// Returns the java home, if `$JAVA_HOME` is defined,
/// it gets returned, otherwise, this function will manually
/// look for a path that contains the `java` binary and returns it.
/// 
/// If any of the previous conditions have been met, the java home will
/// be returned wrapped in a `Some()` variant, if not, `None` will be 
/// returned.
pub fn get_java_home_path() -> Option<PathBuf> {
    match std::env::var("JAVA_HOME") {
        Ok(java_home) => {
            // If the $JAVA_HOME var exists, return it. 
            Some(PathBuf::from(java_home))
        }
        Err(_) => {
            // If not, manually search for a path that contains the 'java' 
            // binary.
            let sep = if cfg!(windows) {
                ';'
            } else {
                ':'
            };

            let path = std::env::var("PATH").unwrap();

            let java_home = path.split(sep).find(|p| {
                let java_path = PathBuf::from(p).join("java");
                match std::fs::metadata(&java_path) {
                    Ok(j) => {
                        j.is_file()
                    }
                    Err(_) => {
                        false
                    }
                }
            });

            match java_home {
                Some(path) => Some(PathBuf::from(path)),
                None => None
            }
        }
    }
}
