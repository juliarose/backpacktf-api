use serde::{Deserialize, Serialize};
use crate::currency_type::CurrencyType;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct MinMax {
    pub currency: CurrencyType,
    pub min: u32,
    pub max: u32,
}