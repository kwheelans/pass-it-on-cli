//! # pass-it-on-cli
//! A command line tool to send notifications to a pass-it-on server
//!
//! ## Usage
//! By providing path to a valid pass-it-on client configuration file one of more messages can be sent with the provided notification name.
//!
//! `pass-it-on-cli -c path/to/client/configuration/file -n my_notification_name -m "message 1" -m "A second message"`
//!
//! ### Client Configuration Example
//! ```toml
//! [client]
//! key = "sdfsf4633ghf44dfhdfhQdhdfhewaasg"
//!
//! [[client.interface]]
//! type = "pipe"
//! path = '/path/to/pipe.fifo'
//! group_read_permission = true
//! group_write_permission = true
//!
//! [[client.interface]]
//! type = "http"
//! host = "192.168.1.2"
//! port = 8080
//! ```

use crate::cli::CliArgs;
use crate::error::Error;
use clap::Parser;
use log::{debug, error, info, LevelFilter};
use pass_it_on::notifications::{ClientReadyMessage, Message};
use pass_it_on::{start_client_arc, ClientConfiguration};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::watch;

mod cli;
mod error;

const LOG_TARGET: &str = "pass-it-on-cli";
const WAIT_BEFORE_SHUTDOWN: u64 = 2000;
const WAIT_AFTER_SHUTDOWN: u64 = 400;

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    simple_logger::SimpleLogger::new()
        .with_level(LevelFilter::Off)
        .env()
        .with_module_level(pass_it_on::LIB_LOG_TARGET, args.verbosity)
        .with_module_level(LOG_TARGET, args.verbosity)
        .with_colors(true)
        .init()
        .unwrap();

    if let Err(error) = run(args).await {
        error!(target: LOG_TARGET, "{}", error)
    } else {
        info!(target: LOG_TARGET, "Done")
    }
}

async fn run(args: CliArgs) -> Result<(), Error> {
    let (shutdown_tx, shutdown_rx) = watch::channel(false);
    let client_arc: Arc<Mutex<Vec<ClientReadyMessage>>> = Arc::new(Mutex::new(Vec::new()));
    let message_arc = Arc::clone(&client_arc);

    let default_config_dir = directories::ProjectDirs::from("com", "pass-it-on", "pass-it-on-cli")
        .unwrap()
        .config_dir()
        .to_path_buf();
    let default_config_path = default_config_dir.join("config.toml");
    debug!(target: LOG_TARGET,"Default Configuration File: {}", default_config_path.to_string_lossy());

    let client_config_path = args.client_config.unwrap_or(default_config_path);
    let client_config =
        ClientConfiguration::try_from(std::fs::read_to_string(client_config_path)?.as_str())?;

    // Process messages and send shutdown signal
    tokio::spawn(async move {
        info!(target: LOG_TARGET, "Processing messages");
        for message in args.messages {
            let msg = Message::new(message).to_client_ready_message(&args.notification_name);
            message_arc.lock().unwrap().push(msg);
        }
        tokio::time::sleep(Duration::from_millis(WAIT_BEFORE_SHUTDOWN)).await;
        if let Err(error) = shutdown_tx.send(true) {
            error!(target: LOG_TARGET, "Unable to send shutdown signal: {}", error)
        }
        tokio::time::sleep(Duration::from_millis(WAIT_AFTER_SHUTDOWN)).await;
    });

    // Start pass-it-on client
    start_client_arc(client_config, client_arc, Some(shutdown_rx), Some(1)).await?;

    Ok(())
}
