use crate::time::ServerTime;
use serde::{Serialize, Deserialize};
use chrono::serde::ts_seconds;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserAgent {
    pub client: String,
    #[serde(with = "ts_seconds")]
    pub last_pulse: ServerTime,
}