use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::response::deserializers::{
    deserialize_attributes
};

pub type Attributes = HashMap<u32, Attribute>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum AttributeValue {
    Number(u64),
    String(String),
    Float(f64),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Attribute {
    pub defindex: u32,
    pub value: Option<AttributeValue>,
    pub float_value: Option<AttributeValue>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_attributes")]
    pub attributes: Attributes,
}