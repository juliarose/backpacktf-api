//! Notification.

mod bundle;
mod contents;

pub use bundle::Bundle;
pub use contents::Contents;

use crate::SteamID;
use crate::time::ServerTime;
use serde::{Serialize, Deserialize};
use chrono::serde::ts_seconds;

/// A notification.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    /// The ID of the notification.
    pub id: String,
    /// The ID of the user who sent the notification.
    pub steamid: SteamID,
    /// Whether the notification is unread.
    #[serde(default)]
    pub unread: bool,
    /// The time the notification was last moved.
    #[serde(with = "ts_seconds")]
    pub last_moved: ServerTime,
    /// The HTML element ID.
    pub element_id: String,
    /// The user ID of the notification.
    pub user_id: SteamID,
    /// The bundle of the notification.
    pub bundle: Bundle,
    /// The contents of the notification.
    pub contents: Contents,
}