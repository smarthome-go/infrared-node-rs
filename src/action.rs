use std::vec;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Action {
    pub name: String,
    pub triggers: Vec<u64>,
    pub homescript: String,
}

impl Default for Action {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            triggers: vec![101],
            homescript: "print('Homescript')".to_string(),
        }
    }
}

pub struct ActionExecRes<'act> {
    pub result: smarthome_sdk_rs::HomescriptExecResponse,
    pub name: &'act str,
}

pub async fn test_setup<'act>(
    actions: &'act Vec<Action>,
    client: &smarthome_sdk_rs::Client,
) -> Result<Vec<ActionExecRes<'act>>, smarthome_sdk_rs::Error> {
    let mut results = vec![];

    for action in actions {
        results.push(ActionExecRes {
            result: client
                .exec_homescript_code(action.homescript.clone(), vec![], true)
                .await?,
            name: &action.name,
        })
    }

    Ok(results)
}

#[inline]
pub fn match_code(actions: &[Action], code: u64) -> Option<&Action> {
    actions
        .iter()
        .find(|action| action.triggers.iter().any(|cod| *cod == code))
}
