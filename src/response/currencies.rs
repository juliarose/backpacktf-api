//! Currencies used in responses.

use crate::error::TryFromResponseCurrenciesError;
use std::fmt;
use std::cmp::{Ord, Ordering};
use serde::{Serialize, Deserializer};
use serde::de::{self, MapAccess};
use serde_json::Value;
use tf2_price::{FloatCurrencies, Currencies, Currency};

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
    /// In-game currencies which includes a hat value.
    InGameWithHat {
        /// Keys.
        keys: f32,
        /// Metal.
        metal: f32,
        /// Hat value.
        hat: f32,
    },
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
            ResponseCurrencies::InGameWithHat { .. } => Err(TryFromResponseCurrenciesError::IsInGameWithHat),
            ResponseCurrencies::Cash(_) => Err(TryFromResponseCurrenciesError::IsCash),
        }
    }
}

impl fmt::Display for ResponseCurrencies {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResponseCurrencies::InGame(currencies) => write!(f, "{currencies}"),
            ResponseCurrencies::InGameWithHat { keys, metal, hat } => {
                let currencies = FloatCurrencies {
                    keys: *keys,
                    metal: *metal,
                };
                
                if currencies.is_empty() {
                    write!(f, "{hat} hat")
                } else {
                    write!(f, "{currencies}, {hat} hat")
                }
            },
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
            ResponseCurrencies::InGameWithHat {
                keys,
                metal,
                hat,
            } => {
                let currencies = FloatCurrencies {
                    keys: *keys,
                    metal: *metal,
                };
                
                if let ResponseCurrencies::InGameWithHat {
                    keys: other_keys,
                    metal: other_metal,
                    hat: other_hat,
                } = other {
                    let other_currencies = FloatCurrencies {
                        keys: *other_keys,
                        metal: *other_metal,
                    };
                    
                    if currencies == other_currencies {
                        if hat == other_hat {
                            Ordering::Equal
                        } else if hat < other_hat {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    } else {
                        currencies.cmp(&other_currencies)
                    }
                } else {
                    // prefer in-game currencies
                    Ordering::Greater
                }
            },
            ResponseCurrencies::Cash(currencies) => {
                if let ResponseCurrencies::Cash(other) = other {
                    if currencies == other {
                        Ordering::Equal
                    } else if currencies < other {
                        Ordering::Less
                    } else {
                        Ordering::Greater
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
            ResponseCurrencies::InGameWithHat {
                keys,
                metal,
                hat,
            } => {
                use serde::ser::SerializeStruct;
                
                let mut s = serializer.serialize_struct("Currencies", 3)?;
                
                if *keys == 0.0 {
                    s.skip_field("keys")?;
                } else if keys.fract() == 0.0 {
                    s.serialize_field("metal", &(*keys as Currency))?;
                } else {
                    s.serialize_field("keys", &keys)?;
                }
                
                if *metal == 0.0 {
                    s.skip_field("metal")?;
                } else if metal.fract() == 0.0 {
                    s.serialize_field("metal", &(*metal as Currency))?;
                } else {
                    s.serialize_field("metal", metal)?;
                }
                
                if *hat == 0.0 {
                    s.skip_field("hat")?;
                } else if metal.fract() == 0.0 {
                    s.serialize_field("hat", &(*hat as Currency))?;
                } else {
                    s.serialize_field("hat", &hat)?;
                }
                
                s.end()
            }
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
                let mut hat = None;
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
                    
                    // I doubt a response would contain all of these fields at the same time
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
                        "hat" => {
                            hat = Some(float_value);
                            contains_value = true;
                        },
                        _ => {
                            return Err(de::Error::custom(format!("invalid currency type: `{currency_name}`")));
                        },
                    }
                }
                
                if let Some(hat) = hat {
                    return Ok(ResponseCurrencies::InGameWithHat {
                        keys: float_currencies.keys,
                        metal: float_currencies.metal,
                        hat,
                    });
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
