use std::path::PathBuf;
use jaburepo::repository::Repository;
use clap::Parser;

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Config {
    pub jabu_repo: Repository,
    pub port: u32
}

#[derive(Clone, PartialEq, Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// The path to the jabu repository.
    #[arg(short, long)]
    pub repo_path: String,

    /// Port to be used by the server.
    #[arg(short, long, default_value = "8080")]
    pub port: u32
}

impl Into<Config> for CliArgs {
    fn into(self) -> Config {
        Config {
            jabu_repo: Repository {
                base_path: self.repo_path.into()
            },
            port: self.port
        }
    }
}

