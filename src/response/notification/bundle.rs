use serde::{Serialize, Deserialize};
use crate::response::listing::{Listing};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Bundle {
    pub listing: Option<Listing>,
}