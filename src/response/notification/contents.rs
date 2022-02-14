use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Contents {
    pub subject: String,
    pub message: String,
    pub url: String,
}