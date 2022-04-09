use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::{
    request::serializers::option_float_as_integers_when_whole,
    response::deserializers::{
        deserialize_attributes,
        string_or_number,
        attribute_value,
        from_optional_float_or_string
    }
};

pub type Attributes = HashMap<i32, Attribute>;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(untagged)]
pub enum Value {
    Number(u64),
    String(String),
}

pub type FloatValue = f64;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Attribute {
    #[serde(deserialize_with = "string_or_number")]
    pub defindex: i32,
    #[serde(default)]
    #[serde(deserialize_with = "attribute_value")]
    pub value: Option<Value>,
    #[serde(default)]
    #[serde(serialize_with = "option_float_as_integers_when_whole", deserialize_with = "from_optional_float_or_string")]
    pub float_value: Option<FloatValue>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_attributes")]
    pub attributes: Attributes,
}