use std::sync::Arc;

use log::{debug, error, info, trace};
use tokio::task;

use crate::action::{match_code, Action};

pub async fn start_scan(
    client: smarthome_sdk_rs::Client,
    device: infrared_rs::Scanner,
    actions: &Vec<Action>,
) -> Result<(), infrared_rs::Error> {
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
                    action.code, res.output,
                );
            }
            false => {
                error!(
                    "HMS execution: action@{} failed with errors:\n{}",
                    action.code,
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
