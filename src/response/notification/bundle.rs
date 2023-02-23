use crate::response::listing::Listing;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Bundle {
    pub listing: Option<Listing>,
}