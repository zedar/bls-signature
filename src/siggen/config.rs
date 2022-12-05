//! Sign message with a secret key

use clap::{ArgGroup, Parser};

/// Config represents configuration for message signing
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
#[command(group(ArgGroup::new("msgs").required(true).args(["msg", "msg_file"])))]
pub struct Config {
    #[arg(short = 's', long = "sk-path")]
    pub sk_file: std::path::PathBuf,

    #[arg(long = "msg")]
    pub msg: Option<String>,

    #[arg(long = "msg-file")]
    pub msg_file: Option<std::path::PathBuf>,

    #[arg(long = "sig-file")]
    pub sig_file: std::path::PathBuf,

    #[arg(short = 'o', long = "overwrite", default_value_t = false)]
    pub overwrite: bool,
}
