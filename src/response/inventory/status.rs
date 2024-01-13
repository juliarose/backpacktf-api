use crate::time::ServerTime;
use serde::{Deserialize, Serialize};
use chrono::serde::ts_seconds;

/// The status of the inventory.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Status {
    /// The current time.
    #[serde(with = "ts_seconds")]
    pub current_time: ServerTime,
    /// The time the inventory was last updated.
    #[serde(with = "ts_seconds")]
    pub last_update: ServerTime,
    /// The timestamp.
    #[serde(with = "ts_seconds")]
    pub timestamp: ServerTime,
    /// The time the inventory can be updated again.
    #[serde(with = "ts_seconds")]
    pub next_update: ServerTime,
    /// The inventory refresh interval.
    pub refresh_interval: u32,
}
