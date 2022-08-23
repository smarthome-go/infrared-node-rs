use std::process;

use clap::Parser;
use config::Error;
use log::{debug, error, warn};

use infrared_rs::Scanner;

mod action;
mod config;
mod scanner;

#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about,
    long_about = "Raspberry-Pi microservice for Smarthome that allows IR control"
)]
struct Args {
    /// The path where the configuration file should be located
    #[clap(short, long, value_parser)]
    config_path: Option<String>,

    /// Discover mode is used to set up new buttons of a remote
    #[clap(short, long, value_parser)]
    discover: bool,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Args::parse();

    let config_path = match args.config_path {
        Some(v) => v,
        None => "/etc/inrs/config.toml".to_string(),
    };

    // Read or create config file
    let conf = match config::read_config(&config_path) {
        Ok(c) => c,
        Err(e) => {
            error!(
                "Could not read or create config file (at {}): {}",
                config_path,
                match e {
                    Error::IO(e) => format!("IO error: {e}"),
                    Error::Parse(e) => format!("invalid TOML syntax: {e}"),
                }
            );
            process::exit(1);
        }
    };

    // Create the Smarthome SDK client
    let client = match smarthome_sdk_rs::Client::new(
        &conf.smarthome.url,
        smarthome_sdk_rs::Auth::QueryToken(conf.smarthome.token),
    )
    .await
    {
        Ok(c) => c,
        Err(e) => {
            error!("Could not establish Smarthome connection: {:?}", e);
            process::exit(1);
        }
    };

    // Execute all action Homescripts in order to validate their correctness
    match action::test_setup(&conf.actions, &client).await {
        Ok(results) => {
            for res in results {
                match res.result.success {
                    true => {
                        debug!(
                            "Check successful: Homescript of action@{} is working",
                            res.code
                        );
                    }
                    false => {
                        error!(
                            "Check failed: Homescript of action@{} contains potential issues:\n{}",
                            res.code,
                            res.result
                                .errors
                                .into_iter()
                                .map(|r| r.to_string())
                                .collect::<Vec<String>>()
                                .join("\n")
                        )
                    }
                }
            }
        }
        Err(e) => error!(
            "Could not test actions Homescript code: Smarthome error: {:?}",
            e
        ),
    };

    // If hardware is disabled, stop here
    if !conf.hardware.enabled {
        warn!("Hardware is currently disabled, exiting...");
        process::exit(0);
    }

    // Create a new scanner
    let scanner = match Scanner::try_new(conf.hardware.pin) {
        Ok(s) => s,
        Err(e) => {
            error!("Could not initialize scanner hardware: {e}");
            process::exit(1)
        }
    };

    match args.discover {
        true => {
            // Start the discovery function
            if let Err(err) = scanner::start_discover(scanner).await {
                error!("Scanner failed unexpectedly: {err}");
                process::exit(1);
            }
        }
        false => {
            // Start the blocking scanner loop
            if let Err(err) = scanner::start_scan(client, scanner, &conf.actions).await {
                error!("Scanner failed unexpectedly: {err}");
                process::exit(1);
            }
        }
    }
}
