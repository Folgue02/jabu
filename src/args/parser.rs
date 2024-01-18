use crate::args::options::*;
use std::collections::{HashMap, HashSet};

pub type ArgParsingResult<R> = Result<R, HashSet<InvalidArgError>>;

#[derive(PartialEq, Debug, Eq, Hash)]
pub enum InvalidArgError {
    MissingOption(String),
    MissingOptionArgument(String),
    UnrecognizedOption(String),
}

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
    pub fn new_with_options(args: Vec<String>, options: &Options) -> ArgParsingResult<Self> {
        let parsed_args = Self::from(args);
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

    /// Checks that all the rules stablished by the <code>options</code> argument
    /// apply. If all of them are applied, `Ok(())` gets returned, otherwise,
    /// a `Vec<InvalidArgError>` gets returned, containing all the errors.
    pub fn validate(&self, options: &Options) -> ArgParsingResult<()> {
        let mut errors: HashSet<InvalidArgError>;

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
            self.options.iter()
                .filter(|(option_name, _)| !options.has_option_with_name(option_name))
                .map(|(extra_option_name, _)| InvalidArgError::UnrecognizedOption(extra_option_name.to_string()))
        );

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
