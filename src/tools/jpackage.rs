use crate::args::parser::InvalidArgError;
use std::path::{PathBuf, Path};

/// Represents the configuration for the `jpackage` cli tool 
/// arguments. Serves as a wrapper for the tool.
#[derive(Debug, PartialEq)]
pub struct JPackageToolConfig {
    /// Path to the target jar to use to create
    /// the self-contained application.
    jar_path: PathBuf,

    /// Name of the application
    app_name: String,

    /// Main class of the jar
    main_class: String,

    /// The output directory in where to output
    /// the self-contained application
    output_dir: String,

    /// Type of the package to create. (*i.e. app-image,
    /// rpm, deb, msi, exem msi*)
    ///
    /// # Note
    /// The value of this field is platform dependant.
    output_type: Option<String>
}

impl JPackageToolConfig {
    pub fn new(
            jar_path: PathBuf,
            app_name: String,
            main_class: String,
            output_dir: String,
            output_type: Option<String>
        ) -> Self {
        Self {
            jar_path,
            app_name,
            main_class,
            output_dir,
            output_type
        }
    }
}

impl TryInto<Vec<String>> for JPackageToolConfig {
    type Error = InvalidArgError;
    fn try_into(self) -> Result<Vec<String>, Self::Error> {
        let mut args = Vec::new();
        let jar_file_name = match self.jar_path.file_name() {
            Some(file_name) => file_name.to_string_lossy().to_string(),
            None => {
                return Err(
                    InvalidArgError::InvalidOptionValue{
                        option_name: "main-jar".to_string(),
                        error_msg: format!("The location of the jar its empty: {:?}", self.jar_path),
                    }
                )
            }
        };
        
        args.push("--input".to_string());
        args.push(
            self.jar_path.parent()
                .unwrap_or(Path::new("."))
                .to_string_lossy()
                .to_string()
        );

        args.push("--main-jar".to_string());
        args.push(jar_file_name);

        args.push("--main-class".to_string());
        args.push(self.main_class);

        args.push("--name".to_string());
        args.push(self.app_name);

        args.push("--dest".to_string());
        args.push(self.output_dir);

        args.push("--type".to_string());
        if let Some(output_type) = self.output_type {
            args.push(output_type);
        } else {
            args.push(
                if cfg!(windows) {
                    "exe"
                } else if cfg!(linux) {
                    "app-image"
                } else {
                    // MacOS has 'dmg', but there could be other
                    // platforms, so 'dmg' might not be the best
                    // output type, but it will cover most use cases.
                    "dmg"
                }.to_string()
            );
        }

        args.push("--verbose".to_string());

        Ok(args)
    }
}

impl JPackageToolConfig {
    /// Generate the arguments for the `jpackage`
    /// cli tool, consuming itself.
    ///
    /// # Note
    /// This method might fail, returning an [`InvalidArgError`], 
    /// this can be caused when the path of the jar path.
    ///
    pub fn try_into_args(self) -> Result<Vec<String>, InvalidArgError> {
        self.try_into()
    }
}
