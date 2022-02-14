use serde::{Deserialize, Serialize};
use crate::response::attributes::AttributeValue;

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ItemAttribute {
    pub defindex: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<AttributeValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub float_value: Option<AttributeValue>,
}

