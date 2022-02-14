use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Value {
    pub raw: f32,
    pub short: String,
    pub long: String,
}