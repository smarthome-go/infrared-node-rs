use infrared_rs::{Error, Scanner};
use log::{debug, error, info, trace};
use smarthome_sdk_rs::Client;
use std::{collections::HashMap, sync::Arc};
use tokio::task;

use crate::action::{match_code, Action};

pub async fn start_discover(device: Scanner) -> Result<(), Error> {
    println!("Press the desired button to get started.\nHint: the most likely correct code will be selected automatically.");
    // Store each result in a hash map to keep track of the most common code
    let mut result_set: HashMap<u64, u8> = HashMap::new();
    // Allow infrared input 10 times
    for step in 0..10 {
        let code = device.scan_blocking()?;
        *result_set.entry(code).or_default() += 1;
        println!("[{:01}] => {code}", step);
    }
    // Calculate the most common value in the hash map (this is likely the correct code)
    let result = result_set
        .into_iter()
        .max_by_key(|(_, occurrences)| *occurrences)
        .unwrap()
        .0;

    println!("=== Result from inputs ===\n{}", result);
    Ok(())
}

pub async fn start_scan(client: Client, device: Scanner, actions: &[Action]) -> Result<(), Error> {
    debug!("Waiting for infrared input...");

    let c = Arc::new(client);

    loop {
        let code = device.scan_blocking()?;
        debug!("Infrared: received code {code}");

        match match_code(actions, code) {
            Some(a) => {
                info!("Matched received code {code} to action {}", a.name);
                // Spawn a tokio task for the action which is handled by the task without blocking
                // concurrent operation
                task::spawn(handle_action(c.clone(), a.clone()));
            }
            None => {
                debug!("")
            }
        }
    }
}

/// Wrapper which executes the homescript of the provided action
/// Is spawned as a tokio task in order to allow concurrent action execution
async fn handle_action(client: Arc<smarthome_sdk_rs::Client>, action: Action) {
    trace!("Executing HMS code on Smarthome server...");
    match client
        .exec_homescript_code(action.homescript, vec![], false)
        .await
    {
        Ok(res) => match res.success {
            true => {
                debug!(
                    "Successfully executed Homescript of action@{}: {}",
                    action.name, res.output,
                );
            }
            false => {
                error!(
                    "HMS execution: action@{} failed with errors:\n{}",
                    action.name,
                    res.errors
                        .into_iter()
                        .map(|r| r.to_string())
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            }
        },
        Err(e) => error!("Could not execute HMS code: Smarthome error {:?}", e),
    };
}
