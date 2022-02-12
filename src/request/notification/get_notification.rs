use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetNotification {
    pub skip: u32,
    pub limit: u32,
    pub unread: bool,
}