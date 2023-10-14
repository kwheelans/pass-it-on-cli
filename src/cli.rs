use clap::Parser;
use std::path::PathBuf;
use log::LevelFilter;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Path to pass-it-on client configuration file
    #[clap(short, long, value_parser)]
    pub client_config: PathBuf,

    /// Notification name for pass-it-on client to use
    #[clap(short, long, value_parser)]
    pub notification_name: String,

    /// Message(s) for pass-it-on client to send
    #[clap(short, long, value_parser)]
    pub messages: Vec<String>,

    /// Set how verbose logging level should be
    #[clap(short, long, value_enum, default_value = "info")]
    pub verbosity: LevelFilter,
}
