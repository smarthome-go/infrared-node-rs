use std::vec;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Action {
    pub name: String,
    pub code: u64,
    pub homescript: String,
}

impl Default for Action {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            code: 101,
            homescript: "print('Homescript')".to_string(),
        }
    }
}

pub struct ActionExecRes {
    pub result: smarthome_sdk_rs::HomescriptExecResponse,
    pub code: u64,
}

pub async fn test_setup(
    actions: &Vec<Action>,
    client: &smarthome_sdk_rs::Client,
) -> Result<Vec<ActionExecRes>, smarthome_sdk_rs::Error> {
    let mut results = vec![];

    for action in actions {
        results.push(ActionExecRes {
            result: client
                .exec_homescript_code(action.homescript.clone(), vec![], true)
                .await?,
            code: action.code,
        })
    }

    Ok(results)
}

#[inline]
pub fn match_code(actions: &Vec<Action>, code: u64) -> Option<&Action> {
    actions.into_iter().find(|a| a.code == code)
}
