use crate::request::serializers::option_float_as_integers_when_whole;
use crate::response::deserializers;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

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
    #[serde(deserialize_with = "deserializers::string_or_number")]
    pub defindex: i32,
    #[serde(default)]
    #[serde(deserialize_with = "deserializers::attribute_value")]
    pub value: Option<Value>,
    #[serde(default)]
    #[serde(serialize_with = "option_float_as_integers_when_whole", deserialize_with = "deserializers::from_optional_float_or_string")]
    pub float_value: Option<FloatValue>,
    #[serde(default)]
    #[serde(deserialize_with = "deserializers::deserialize_attributes")]
    pub attributes: Attributes,
}