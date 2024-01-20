mod config;
mod tasks;
mod args;

#[cfg(test)]
mod tests;

fn main() {
    let config = config::JabuConfig::default_of_name("test", config::java::ProjectType::Binary);
    println!("{}", ron::ser::to_string_pretty(&config, ron::ser::PrettyConfig::default()).unwrap());
}
