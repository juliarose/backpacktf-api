use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeleteListingsResult {
    pub deleted: u32,
}