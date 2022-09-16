use std::process;

use clap::{Parser, Subcommand};
use config::Error;
use log::{error, warn};

use infrared_rs::Scanner;
use smarthome_sdk_rs::{Auth, Client};

mod action;
mod config;
mod scanner;

#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    /// The path where the configuration file should be located
    #[clap(short, long, value_parser)]
    config_path: Option<String>,

    /// The mode application mode in which ifrs should operate
    #[clap(subcommand)]
    mode: Mode,
}

#[derive(Subcommand)]
enum Mode {
    /// Starts the service in daemon mode, listens for incoming signals
    Run,
    /// Discover mode is used to set up new buttons of a remote
    Discover,
    /// Lint the Homescript code used in the actions
    Lint,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Args::parse();

    // Select the configuration file's path and override it if required
    let config_path = match args.config_path {
        Some(v) => v,
        // Default configuration file location is defined here
        None => "/etc/ifrs/config.toml".to_string(),
    };

    // Read or create the configuration file
    let conf = match config::read_config(&config_path) {
        Ok(conf) => conf,
        Err(err) => {
            error!(
                "Could not read nor create config file (at {config_path}): {}",
                match err {
                    Error::IO(err) => format!("IO error: {err}"),
                    Error::Parse(err) => format!("invalid TOML syntax: {err}"),
                }
            );
            process::exit(1);
        }
    };

    // Create the Smarthome SDK client
    let client =
        match Client::new(&conf.smarthome.url, Auth::QueryToken(conf.smarthome.token)).await {
            Ok(c) => c,
            Err(e) => {
                error!(
                    "Could not create Smarthome client: failed to establish connection: {:?}",
                    e
                );
                process::exit(1);
            }
        };

    // Start the application in different modes depending on the subcommand
    match args.mode {
        Mode::Lint => {
            // Execute all action Homescripts in order to validate their correctness
            match action::lint_actions(&conf.actions, &client).await {
                Ok(results) => {
                    for res in results {
                        match res.result.success {
                            true => {
                                println!(
                                    "Check successful: Homescript of action@{} is working",
                                    res.name
                                );
                            }
                            false => {
                                error!(
                            "Check failed: Homescript of action@{} contains potential issues:\n{}",
                            res.name,
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
                Err(e) => eprintln!(
                    "Could not test actions Homescript code: Smarthome error: {:?}",
                    e
                ),
            };
        }
        mode => {
            // If the hardware is disabled, stop here
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
            // Decide whether to start the discovery or the listen function
            match mode {
                Mode::Discover => {
                    // Start the discovery function
                    if let Err(err) = scanner::start_discover(scanner).await {
                        error!("Scanner failed unexpectedly: {err}");
                        process::exit(1);
                    }
                }
                Mode::Run => {
                    // Start the blocking scanner loop
                    if let Err(err) = scanner::start_scan(client, scanner, &conf.actions).await {
                        error!("Scanner failed unexpectedly: {err}");
                        process::exit(1);
                    }
                }
                _ => unreachable!("Other modes should have been checked beforehand"),
            }
        }
    }
}
