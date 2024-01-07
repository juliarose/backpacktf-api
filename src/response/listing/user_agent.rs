use crate::time::ServerTime;
use serde::{Serialize, Deserialize};
use chrono::serde::ts_seconds;

/// The user agent the listing is managed by.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserAgent {
    /// The client string.
    pub client: String,
    /// The time when the last pulse was sent at.
    #[serde(with = "ts_seconds")]
    pub last_pulse: ServerTime,
}