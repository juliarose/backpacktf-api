use crate::time::ServerTime;
use serde::{Serialize, Deserialize};
use chrono::serde::ts_seconds;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AgentStatus {
    pub status: String,
    #[serde(with = "ts_seconds")]
    pub current_time: ServerTime,
    #[serde(with = "ts_seconds")]
    pub expire_at: ServerTime,
    pub client: String,
}