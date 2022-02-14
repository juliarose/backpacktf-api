use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Cursor {
    pub skip: u32,
    pub total: u32,
    pub limit: u32,
}