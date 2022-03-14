use serde::{Deserialize, Serialize};
use crate::request::currencies::Currencies;

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct UpdateListing {
    pub details: Option<String>,
    pub currencies: Currencies,
}