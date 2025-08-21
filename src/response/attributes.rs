//! Attributes.

use crate::request::serializers::option_float_as_integers_when_whole;
use crate::response::deserializers;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// A list of attributes.
pub type Attributes = HashMap<i32, Attribute>;

/// A float value.
pub type FloatValue = f32;
/// An integer.
pub type Integer = u32;

/// The value of an attribute.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(untagged)]
pub enum Value {
    /// An integer value.
    Integer(Integer),
    /// A float value.
    Float(FloatValue),
    /// A string value.
    String(String),
}

impl From<tf2_enum::AttributeValue> for Value {
    fn from(value: tf2_enum::AttributeValue) -> Self {
        match value {
            tf2_enum::AttributeValue::Integer(n) => Value::Integer(n),
            tf2_enum::AttributeValue::Float(f) => Value::Float(f),
            tf2_enum::AttributeValue::String(s) => Value::String(s),
            tf2_enum::AttributeValue::None => Value::Integer(0),
        }
    }
}

impl From<Integer> for Value {
    fn from(value: Integer) -> Self {
        Value::Integer(value)
    }
}

impl From<FloatValue> for Value {
    fn from(value: FloatValue) -> Self {
        Value::Float(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        (value as FloatValue).into()
    }
}

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
