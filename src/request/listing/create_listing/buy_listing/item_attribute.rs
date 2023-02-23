use crate::response::attributes::{Value as AttributeValue, FloatValue};
use crate::request::serializers::option_float_as_integers_when_whole;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ItemAttribute {
    pub defindex: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<AttributeValue>,
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "option_float_as_integers_when_whole")]
    pub float_value: Option<FloatValue>,
}

