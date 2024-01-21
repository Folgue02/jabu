use std::path::PathBuf;


fn which_javac() -> Option<String> {
    match std::env::var("JAVA_HOME") {
        Ok(java_home) => {
            if let Ok(javac_file) = std::fs::metadata(PathBuf::from(java_home.to_owned()).join("javac")) {
                if !javac_file.is_file() {
                    dbg!("The javac binary in the $JAVA_HOME is not a file. {}", javac_file);
                    None
                } else {
                    Some(PathBuf::from(java_home).join("javac").to_string_lossy().to_string())
                }
            } else {
                None
            }
        }
        Err(_) => None,
    }   
}