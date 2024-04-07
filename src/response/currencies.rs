//! Currencies used in responses.

use crate::error::TryFromResponseCurrenciesError;
use std::fmt;
use std::cmp::{Ord, Ordering};
use serde::{Serialize, Deserializer};
use serde::de::{self, MapAccess};
use serde_json::Value;
use tf2_price::{FloatCurrencies, Currencies};

/// Generally, this will be in-game currencies, but it can also be cash currencies for 
/// marketplace.tf cross-listings.
/// 
/// It is best to use this as an intermediate type for converting into [`Currencies`] as it 
/// provides a safe way to handle currencies.
/// 
/// # Examples
/// ```
/// use backpacktf_api::response::currencies::ResponseCurrencies;
/// use tf2_price::{Currencies, FloatCurrencies, refined};
/// 
/// let response_currencies = ResponseCurrencies::InGame(FloatCurrencies {
///     keys: 2.0,
///     metal: 1.0,
/// });
/// let currencies: Currencies = response_currencies.try_into().unwrap();
/// 
/// assert_eq!(currencies.keys, 2);
/// assert_eq!(currencies.weapons, refined!(1));
/// ````
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ResponseCurrencies {
    /// In-game currencies.
    InGame(FloatCurrencies),
    /// Cash currencies for marketplace.tf cross-listings.
    Cash(f32),
}

impl Eq for ResponseCurrencies {}

impl Into<ResponseCurrencies> for FloatCurrencies {
    fn into(self) -> ResponseCurrencies {
        ResponseCurrencies::InGame(self)
    }
}

impl TryFrom<ResponseCurrencies> for Currencies {
    type Error = TryFromResponseCurrenciesError;
    
    fn try_from(currencies: ResponseCurrencies) -> Result<Self, Self::Error> {
        match currencies {
            ResponseCurrencies::InGame(currencies) => Ok(currencies.try_into()?),
            ResponseCurrencies::Cash(_) => Err(TryFromResponseCurrenciesError::IsCash),
        }
    }
}

impl fmt::Display for ResponseCurrencies {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResponseCurrencies::InGame(currencies) => write!(f, "{currencies}"),
            ResponseCurrencies::Cash(currencies) => write!(f, "{currencies}"),
        }
    }
}

impl PartialOrd for ResponseCurrencies {
    fn partial_cmp(&self, other: &ResponseCurrencies) -> Option<Ordering> {
       Some(self.cmp(other))
    }
}

impl Ord for ResponseCurrencies {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            ResponseCurrencies::InGame(currencies) => {
                if let ResponseCurrencies::InGame(other) = other {
                    currencies.cmp(other)
                } else {
                    Ordering::Less
                }
            },
            ResponseCurrencies::Cash(currencies) => {
                if let ResponseCurrencies::Cash(other) = other {
                    if currencies == other {
                        Ordering::Equal
                    } else if currencies < other {
                        Ordering::Less
                    } else if currencies > other {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                } else {
                    // prefer in-game currencies
                    Ordering::Greater
                }
            },
        }
    }
}

impl ResponseCurrencies {
    /// Checks if the currencies are in-game currencies.
    pub fn is_in_game(&self) -> bool {
        matches!(self, ResponseCurrencies::InGame(_))
    }
}

impl PartialEq<FloatCurrencies> for ResponseCurrencies {
    fn eq(&self, other: &FloatCurrencies) -> bool {
        match self {
            ResponseCurrencies::InGame(currencies) => currencies == other,
            _ => false,
        }
    }
}

impl PartialEq<tf2_price::Currencies> for ResponseCurrencies {
    fn eq(&self, other: &tf2_price::Currencies) -> bool {
        match self {
            ResponseCurrencies::InGame(currencies) => currencies == other,
            _ => false,
        }
    }
}

impl Serialize for ResponseCurrencies {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        #[derive(Serialize)]
        struct SerializeUSD<'a> {
            usd: &'a f32,
        }
        
        match self {
            ResponseCurrencies::InGame(currencies) => {
                // Should serialize nicely with no decimals on whole numbers
                currencies.serialize(serializer)
            },
            ResponseCurrencies::Cash(usd) => {
                SerializeUSD { usd }.serialize(serializer)
            },
        }
    }
}

impl<'de> serde::Deserialize<'de> for ResponseCurrencies {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        struct ResponseCurrenciesVisitor;
        
        impl<'de> de::Visitor<'de> for ResponseCurrenciesVisitor {
            type Value = ResponseCurrencies;
        
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an integer or a string")
            }
            
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut float_currencies = FloatCurrencies::new();
                let mut contains_value = false;
                
                while let Some((currency_name, value)) = access.next_entry::<String, Value>()? {
                    let float_value = match value {
                        Value::Number(number) => {
                            number.as_f64().ok_or_else(|| de::Error::custom("invalid number"))? as f32
                        },
                        Value::String(s) => {
                            s.parse::<f32>().map_err(de::Error::custom)?
                        },
                        _ => {
                            return Err(de::Error::custom("invalid currency value"));
                        },
                    };
                    
                    // I doubt a response would contain all three of these fields at the same time
                    match currency_name.as_str() {
                        "keys" => {
                            float_currencies.keys = float_value;
                            contains_value = true;
                        },
                        "metal" => {
                            float_currencies.metal = float_value;
                            contains_value = true;
                        },
                        "usd" => {
                            return Ok(ResponseCurrencies::Cash(float_value));
                        },
                        _ => {
                            return Err(de::Error::custom(format!("invalid currency type: `{currency_name}`")));
                        },
                    }
                }
                
                if !contains_value {
                    return Err(de::Error::custom("no currency value found"));
                }
                
                Ok(float_currencies.into())
            }
        }
        
        deserializer.deserialize_any(ResponseCurrenciesVisitor)
    }
}