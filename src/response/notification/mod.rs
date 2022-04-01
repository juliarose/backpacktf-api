mod bundle;
mod contents;

pub use bundle::Bundle;
pub use contents::Contents;

use serde::{Serialize, Deserialize};
use chrono::serde::ts_seconds;
use crate::time::ServerTime;
use steamid_ng::SteamID;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub id: String,
    pub steamid: SteamID,
    #[serde(default)]
    pub unread: bool,
    #[serde(with = "ts_seconds")]
    pub last_moved: ServerTime,
    pub element_id: String,
    pub user_id: SteamID,
    pub bundle: Bundle,
    pub contents: Contents,
}