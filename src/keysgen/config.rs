//! # Secret/public key generator configuration

use clap::Parser;

/// Config represents configuration of the secret and public key generator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
pub struct Config {
    #[arg(short = 't', long = "target-path")]
    pub target_path: std::path::PathBuf,

    #[arg(short = 'o', long = "overwrite", default_value_t = false)]
    pub overwrite: bool,
}
