use serde::{Deserialize, Serialize};
use crate::time::{ServerTime};
use chrono::serde::{ts_seconds};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Status {
    #[serde(with = "ts_seconds")]
    pub current_time: ServerTime,
    #[serde(with = "ts_seconds")]
    pub last_update: ServerTime,
    #[serde(with = "ts_seconds")]
    pub timestamp: ServerTime,
    #[serde(with = "ts_seconds")]
    pub next_update: ServerTime,
    pub refresh_interval: u32,
}
