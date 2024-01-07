use crate::response::attributes::{Value as AttributeValue, FloatValue};
use crate::request::serializers::option_float_as_integers_when_whole;
use serde::{Deserialize, Serialize};

/// An item attribute. For a list of attributes see the 
/// [wiki](https://wiki.teamfortress.com/wiki/List_of_item_attributes).
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ItemAttribute {
    /// The defindex of the attribute.
    pub defindex: i32,
    /// The value of the attribute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<AttributeValue>,
    /// The float value of the attribute.
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "option_float_as_integers_when_whole")]
    pub float_value: Option<FloatValue>,
}

