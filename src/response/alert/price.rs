use serde::{Deserialize, Serialize};
use crate::response::deserializers::currency_type_enum_from_str;
use crate::currency_type::CurrencyType;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Price {
    #[serde(deserialize_with = "currency_type_enum_from_str")]
    pub currency: CurrencyType,
    pub min: Option<f32>,
    pub max: Option<f32>,
}