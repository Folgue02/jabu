use crate::{
    args::options::*,
    tasks::TaskError
};
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

pub type ArgParsingResult<R> = Result<R, HashSet<InvalidArgError>>;

#[derive(PartialEq, Debug, Eq, Hash)]
/// Represents errors that might occur while parsing a list of arguments
/// and comparing it to an `Options` object.
pub enum InvalidArgError {
    /// A required option wasn't specified by the user.
    MissingOption(String),

    /// The option was specified, but no argument value was given. The
    /// String in the tuple contains the name of the option.
    MissingOptionArgument(String),

    /// Given when an option contains an invalid value, i.e., the 
    /// option was supposed to contain an integer, instead it had a 
    /// character.
    InvalidOptionValue{option_name: String, error_msg: String},

    /// An non recognized option was specified in the arguments by 
    /// the user. The string of the tuple represents the name of that
    /// unrecognized option.
    UnrecognizedOption(String),
}

impl std::fmt::Display for InvalidArgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::MissingOption(option_name) => {
                format!("Option '{option_name}' not specified.")
            }
            Self::MissingOptionArgument(option_name) => {
                format!("Option '{option_name}' specified, but no argument value was specified with it (it's required)")
            }
            Self::InvalidOptionValue{ option_name, error_msg } => {
                format!("The value specified for option '{option_name}' was not valid: {error_msg}")
            }
            Self::UnrecognizedOption(unrecognized_option) => {
                format!("Unrecognized option '{unrecognized_option}'")
            }
        };
        write!(f, "{}", msg)
    }
}

impl Error for InvalidArgError {}

#[derive(Debug, PartialEq)]
pub struct ParsedArguments {
    pub options: HashMap<String, Option<String>>,
    pub arg_list: Vec<String>,
}

impl From<Vec<String>> for ParsedArguments {
    fn from(args: Vec<String>) -> Self {
        let mut arg_list = Vec::new();
        let mut options = HashMap::new();
        let mut no_parsing = false;

        for arg in args {
            if no_parsing {
                arg_list.push(arg);
                continue;
            }

            if arg == "--" {
                no_parsing = true;
            } else if arg.starts_with("--") {
                if let Some(colon_pos) = arg.find(":") {
                    options.insert(
                        arg[2..colon_pos].to_string(),
                        Some(arg[colon_pos + 1..].to_string()),
                    );
                } else {
                    options.insert(arg[2..].to_string(), None);
                }
            } else {
                arg_list.push(arg);
            }
        }

        Self {
            arg_list: arg_list.iter().map(|a| a.to_string()).collect(),
            options,
        }
    }
}

impl ParsedArguments {
    /// Creates an instance of [`ParsedArguments`] with the given
    /// `args`, while also checking if the provided arguments are valid based on the
    /// given `options`.
    ///
    /// # Arguments
    /// * `args`: The arguments given by the user (*i.e. cli arguments*)
    /// * `options`: The options that will be used to check if the given arguments are
    /// valid.
    ///
    /// # Note
    /// If `--help` is specified, this method will print `options`' help and
    /// then quit with exit code 0.
    pub fn new_with_options(args: Vec<String>, options: &Options) -> ArgParsingResult<Self> {
        let mut parsed_args = Self::from(args);
        
        // BUG: When a task is executed with "--help", and this task has 
        // dependency tasks, the dependency tasks will get executed, and then
        // the print_help() will be executed.
        if parsed_args.has_option_with_name("help") {
            options.print_help();
            // TODO: Should it change? 
            std::process::exit(0);
        }
        match parsed_args.validate(options) {
            Ok(_) => Ok(parsed_args),
            Err(errors) => Err(errors),
        }
    }

    /// Gets the value of of the given option name, if the specified option doesn't
    /// appear in the `options` hashmap, `None` will be returned, otherwise,
    /// an `&Option<String>` containing the value of the option is returned.
    pub fn get_option_value(&self, option_name: &str) -> Option<&Option<String>> {
        self.options.get(option_name)
    }

    pub fn has_option_with_name<T>(&self, option_name: T) -> bool 
        where T: AsRef<str> {
        self.options.contains_key(option_name.as_ref())
    }

    /// Checks that all the rules stablished by the <code>options</code> argument
    /// apply. If all of them are applied, `Ok(())` gets returned, otherwise,
    /// a `Vec<InvalidArgError>` gets returned, containing all the errors.
    pub fn validate(&mut self, options: &Options) -> ArgParsingResult<()> {
        let mut errors: HashSet<InvalidArgError>;

        // Insert the default values
        options.options.iter()
            .filter(|o| o.default_value.is_some())
            .for_each(|defaulted_option| {
                self.options.entry(defaulted_option.name.clone()).or_insert(defaulted_option.default_value.clone());
            });

        errors = options
            .options
            .iter()
            .map(|option| match self.get_option_value(&option.name) {
                Some(option_value) => match option_value {
                    Some(_) => None,
                    None => {
                        if option.has_arg {
                            Some(InvalidArgError::MissingOptionArgument(
                                option.name.to_string(),
                            ))
                        } else {
                            None
                        }
                    }
                },
                None => {
                    if option.required {
                        Some(InvalidArgError::MissingOption(option.name.to_string()))
                    } else {
                        None
                    }
                }
            })
            .filter_map(|invalid_arg_error| invalid_arg_error)
            .collect();

        // Check for extra options in `options`
        errors.extend(
            self.options
                .iter()
                .filter(|(option_name, _)| !options.has_option_with_name(option_name))
                .map(|(extra_option_name, _)| {
                    InvalidArgError::UnrecognizedOption(extra_option_name.to_string())
                }),
        );

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
