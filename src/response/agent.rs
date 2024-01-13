//! Agent status.

use crate::time::ServerTime;
use serde::{Serialize, Deserialize};
use chrono::serde::ts_seconds;

/// The status of the agent.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AgentStatus {
    /// The status of the agent.
    pub status: String,
    /// The current time.
    #[serde(with = "ts_seconds")]
    pub current_time: ServerTime,
    /// The time the status expires at.
    #[serde(with = "ts_seconds")]
    pub expire_at: ServerTime,
    /// The client string.
    pub client: String,
}