//! # pass-it-on-command-line-client
//! A command line tool to sent a single notification to a pass-it-on server

use std::sync::{Arc, Mutex};
use std::time::Duration;
use clap::Parser;
use log::{error, info, LevelFilter};
use pass_it_on::{ClientConfiguration, start_client_arc};
use pass_it_on::notifications::{ClientReadyMessage, Message};
use tokio::sync::watch;
use crate::cli::CliArgs;
use crate::error::Error;

mod cli;
mod error;

const LOG_TARGET: &str = "pio-cmd-line-client";
const WAIT_BEFORE_SHUTDOWN: u64 = 500;
const WAIT_AFTER_SHUTDOWN: u64 = 300;

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
    let client_config = ClientConfiguration::try_from(std::fs::read_to_string(args.client_config)?.as_str())?;
    let (shutdown_tx, shutdown_rx) = watch::channel(false);
    let client_arc: Arc<Mutex<Vec<ClientReadyMessage>>> = Arc::new(Mutex::new(Vec::new()));
    let message_arc = Arc::clone(&client_arc);

    // Process messages and send shutdown signal
    tokio::spawn(async move {
        info!(target: LOG_TARGET, "Processing messages");
        for message in  args.messages {
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
