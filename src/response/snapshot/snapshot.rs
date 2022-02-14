use serde::{Serialize, Deserialize};
use chrono::serde::ts_seconds;
use crate::time::ServerTime;
use super::Listing;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    #[serde(with = "ts_seconds")]
    pub created_at: ServerTime,
    pub appid: Option<u32>,
    pub sku: String,
    #[serde(default)]
    pub listings: Vec<Listing>,
}