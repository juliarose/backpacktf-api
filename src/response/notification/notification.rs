use serde::{Serialize, Deserialize};
use chrono::serde::{ts_seconds};
use crate::time::ServerTime;
use steamid_ng::SteamID;
use super::{NotificationContents, NotificationBundle};

#[derive(Serialize, Deserialize, Clone, Debug)]
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
    pub bundle: NotificationBundle,
    pub contents: NotificationContents,
}