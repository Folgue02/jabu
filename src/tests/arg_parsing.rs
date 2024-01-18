use crate::args::parser::{ParsedArguments, InvalidArgError};
use crate::args::options::{Options, ParOptionBuilder};
use std::collections::{HashMap, HashSet};

#[test]
fn parse_valid_args() {
    let mut options = HashMap::new();
    options.insert("name".to_string(), Some("Daniel".to_string()));
    options.insert("age".to_string(), Some("20".to_string()));
    options.insert("working".to_string(), None);
    let expected = ParsedArguments {
        options,
        arg_list: vec!["--imposed".to_string()]
    };
    let args = vec![
        "--name:Daniel".to_string(),
        "--age:20".to_string(),
        "--working".to_string(),
        "--".to_string(),
        "--imposed".to_string(),
    ];
    let result = ParsedArguments::from(args);

    assert_eq!(expected, result);
}

#[test]
fn parse_empty_values() {
    let raw_args = vec![
        "--name:".to_string(),
    ];
    let mut options = HashMap::new();
    options.insert("name".to_string(), Some(String::new()));
    let arg_list = Vec::new();
    let expected = ParsedArguments {
        options,
        arg_list,
    };
    let result = ParsedArguments::from(raw_args);
    assert_eq!(expected, result);
}

fn get_sample_options() -> Options {
    let mut options = Options::default();
    options.add_option(
        ParOptionBuilder::default()
            .name("name")
            .short('n')
            .has_arg(true)
            .required(true)
            .build()
    );
    options.add_option(
        ParOptionBuilder::default()
            .name("age")
            .short('a')
            .has_arg(true)
            .required(true)
            .build()
    );
    options.add_option(
        ParOptionBuilder::default()
            .name("working")
            .short('w')
            .has_arg(false)
            .required(false)
            .build()
    );
    options
}

#[test]
fn option_parsed_args_validation() {
    let options = get_sample_options();
    let args = vec![
        "--name:Daniel".to_string(),
        "--age:20".to_string(),
        "--working".to_string()
    ];
    assert!(crate::args::parser::ParsedArguments::new_with_options(args, &options).is_ok())
}

#[test]
fn parse_invalid_args() {
    let options = get_sample_options();
    let args = vec![
        "--name".to_string()
    ];
    let parsed_args = ParsedArguments::new_with_options(args, &options);
    match parsed_args {
        Ok(_) => assert!(false, "Invalid arguments parsed as if they were valid."),
        Err(e) => {
            let mut expected = HashSet::new();
            expected.insert(InvalidArgError::MissingOption("age".to_string()));
            expected.insert(InvalidArgError::MissingOptionArgument("name".to_string()));
            assert_eq!(expected, e)
        }
    }
}

#[test]
fn parse_extra_args() {
    let options = get_sample_options();
    let args = vec![
        "--name:Daniel".to_string(),
        "--age:20".to_string(),
        "--working".to_string(),
        "--unnecessary".to_string()
    ];
    let parsed_args = ParsedArguments::new_with_options(args, &options);
    match parsed_args {
        Ok(_) => assert!(false, "Should have thrown a 'UnrecognizedOption' error."),
        Err(e) => {
            let mut expected = HashSet::new();
            expected.insert(InvalidArgError::UnrecognizedOption("unnecessary".to_string()));
            assert_eq!(expected, e)
        }
    }
}