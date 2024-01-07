use crate::request::serializers::option_float_as_integers_when_whole;
use crate::response::deserializers;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub type Attributes = HashMap<i32, Attribute>;

/// The value of an attribute.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(untagged)]
pub enum Value {
    Number(u64),
    Float(f64),
    String(String),
}

pub type FloatValue = f64;

/// An attribute of an item.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Attribute {
    /// The defindex of the attribute.
    #[serde(deserialize_with = "deserializers::string_or_number")]
    pub defindex: i32,
    /// The value of the attribute.
    #[serde(default)]
    #[serde(deserialize_with = "deserializers::attribute_value")]
    pub value: Option<Value>,
    /// The float value of the attribute.
    #[serde(default)]
    #[serde(serialize_with = "option_float_as_integers_when_whole", deserialize_with = "deserializers::from_optional_float_or_string")]
    pub float_value: Option<FloatValue>,
    /// The attributes belonging to this attribute.
    #[serde(default)]
    #[serde(deserialize_with = "deserializers::deserialize_attributes")]
    pub attributes: Attributes,
}