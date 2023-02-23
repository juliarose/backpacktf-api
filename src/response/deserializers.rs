use crate::{ListingIntent, CurrencyType};
use crate::response::listing::Ban;
use crate::response::attributes::{Attributes, Attribute, Value as AttributeValue};
use std::str::FromStr;
use std::fmt::Display;
use std::marker::PhantomData;
use num_enum::TryFromPrimitive;
use serde::Deserialize;
use serde::de::{self, Deserializer, Visitor, MapAccess, SeqAccess, Unexpected, IntoDeserializer};
use serde_json::Value;

pub fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(de::Error::invalid_value(
            Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}

pub fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: std::fmt::Display,
    D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    
    T::from_str(&s).map_err(de::Error::custom)
}

// todo optimize this
pub fn presence<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match Value::deserialize(deserializer)? {
        Value::Null => Ok(false),
        _ => Ok(true),
    }
}

pub fn number_to_u32(value: &serde_json::Number) -> Result<u32, String> {
    let number = value.as_u64()
        .ok_or_else(||"not an integer".to_string())?;
    let number = u32::try_from(number)
        .map_err(|_e| "number does not fit into u32".to_string())?;
    
    Ok(number)
}

pub fn map_to_enum<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: TryFromPrimitive + TryFrom<u32> + FromStr + Deserialize<'de>,
    <T as FromStr>::Err: std::fmt::Display,
    <T as TryFrom<u32>>::Error: std::fmt::Display
{
    match Value::deserialize(deserializer)? {
        Value::Object(map) => {
            if let Some(value) = map.get("id") {
                match value {
                    Value::Number(number) => {
                        let id = number_to_u32(number).map_err(de::Error::custom)?;
                        
                        T::try_from(id).map_err(de::Error::custom)
                    },
                    value => {
                        Err(de::Error::custom(format!("expected a number, got `{value}`")))
                    },
                }
            } else if let Some(name) = map.get("name") {
                match name {
                    Value::String(string) => {
                        T::from_str(string).map_err(de::Error::custom)
                    },
                    value => {
                        Err(de::Error::custom(format!("expected a number, got `{value}`")))
                    },
                }
            } else {
                Err(de::Error::custom(format!("expected a number, got `{}`", 1)))
            }
        },
        Value::Number(number) => {
            let id = number_to_u32(&number).map_err(de::Error::custom)?;
            
            T::try_from(id).map_err(de::Error::custom)
        },
        value => {
            Err(de::Error::custom(format!("expected a number, got `{value}`")))
        },
    }
}

pub fn map_to_enum_option<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: TryFromPrimitive + TryFrom<u32> + FromStr + Deserialize<'de>,
    <T as FromStr>::Err: std::fmt::Display,
    <T as TryFrom<u32>>::Error: std::fmt::Display
{
    match Value::deserialize(deserializer)? {
        Value::Object(map) => {
            if let Some(value) = map.get("id") {
                match value {
                    Value::Number(number) => {
                        let id = number_to_u32(number).map_err(de::Error::custom)?;
                        
                        Ok(Some(T::try_from(id).map_err(de::Error::custom)?))
                    },
                    value => {
                        Err(de::Error::custom(format!("expected a number, got `{value}`")))
                    },
                }
            } else if let Some(name) = map.get("name") {
                match name {
                    Value::String(string) => {
                        Ok(Some(T::from_str(string).map_err(de::Error::custom)?))
                    },
                    value => {
                        Err(de::Error::custom(format!("expected a number, got `{value}`")))
                    },
                }
            } else {
                Err(de::Error::custom(format!("expected a number, got `{}`", 1)))
            }
        },
        Value::Number(number) => {
            let id = number_to_u32(&number).map_err(de::Error::custom)?;
            
            Ok(Some(T::try_from(id).map_err(de::Error::custom)?))
        },
        Value::Null => Ok(None),
        value => {
            Err(de::Error::custom(format!("expected a number, got `{value}`")))
        },
    }
}

pub fn map_to_enum_option_from_name<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: TryFromPrimitive + FromStr + Deserialize<'de>,
    <T as FromStr>::Err: std::fmt::Display,
{
    match Value::deserialize(deserializer)? {
        Value::Object(map) => {
            if let Some(name) = map.get("name") {
                match name {
                    Value::String(string) => {
                        // skip paints which are hexadecimal numbers
                        if string.starts_with('#') {
                            Ok(None)
                        } else {
                            Ok(Some(T::from_str(string).map_err(de::Error::custom)?))
                        }
                    },
                    value => {
                        Err(de::Error::custom(format!("expected a string, got `{value}`")))
                    },
                }
            } else {
                Err(de::Error::custom(format!("expected a string, got `{}`", 1)))
            }
        },
        Value::String(string) => {
            Ok(Some(T::from_str(&string).map_err(de::Error::custom)?))
        },
        Value::Null => Ok(None),
        value => {
            Err(de::Error::custom(format!("expected a string, got `{value}`")))
        },
    }
}

// this is somewhat implicit
pub fn attribute_value<'de, D>(deserializer: D) -> Result<Option<AttributeValue>, D::Error>
where
    D: Deserializer<'de>,
{
    match Value::deserialize(deserializer)? {
        Value::String(s) => {
            if s.is_empty() {
                return Ok(None);
            }
            
            match s.parse::<u64>() {
                Ok(n) => Ok(Some(AttributeValue::Number(n))),
                Err(_) => Ok(Some(AttributeValue::String(s))),
            }
        },
        Value::Number(num) => {
            let n: u64 = num.as_u64().ok_or_else(|| de::Error::custom("invalid number"))?;
            
            Ok(Some(AttributeValue::Number(n)))
        },
        Value::Null => Ok(None),
        _ => Err(de::Error::custom("invalid attribute")),
    }
}

pub fn from_number_or_string<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr + Default,
    T: TryFrom<u64>,
    T::Err: std::fmt::Display,
    D: Deserializer<'de>
{
    match Value::deserialize(deserializer)? {
        Value::String(s) => {
            if s.is_empty() {
                return Ok(T::default());
            }
            
            let n = s.parse::<T>().map_err(de::Error::custom)?;
                
            Ok(n)
        },
        Value::Number(num) => {
            let n: u64 = num.as_u64().ok_or_else(|| de::Error::custom("invalid number"))?;
            
            match T::try_from(n) {
                Ok(c) => {
                    Ok(c)
                },
                Err(_e) => {
                    Err(de::Error::custom("number too large to fit in target type"))
                }
            }
        },
        Value::Null => Ok(T::default()),
        _ => Err(de::Error::custom("not a number")),
    }
}

pub fn from_optional_number_or_string_integer<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: FromStr,
    T: TryFrom<i64>,
    T::Err: std::fmt::Display,
    D: Deserializer<'de>
{
    match Value::deserialize(deserializer)? {
        Value::String(s) => {
            if s.is_empty() {
                return Ok(None);
            }
            
            let n = s.parse::<T>().map_err(de::Error::custom)?;
                
            Ok(Some(n))
        },
        Value::Number(num) => {
            let n: i64 = num.as_i64().ok_or_else(|| de::Error::custom("invalid number"))?;
            
            match T::try_from(n) {
                Ok(c) => {
                    Ok(Some(c))
                },
                Err(_e) => {
                    Err(de::Error::custom("number too large to fit in target type"))
                }
            }
        },
        Value::Null => Ok(None),
        _ => Err(de::Error::custom("not a number")),
    }
}

pub fn from_optional_number_or_string<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: FromStr,
    T: TryFrom<u64>,
    T::Err: std::fmt::Display,
    D: Deserializer<'de>
{
    match Value::deserialize(deserializer)? {
        Value::String(s) => {
            if s.is_empty() {
                return Ok(None);
            }
            
            let n = s.parse::<T>().map_err(de::Error::custom)?;
                
            Ok(Some(n))
        },
        Value::Number(num) => {
            let n: u64 = num.as_u64().ok_or_else(|| de::Error::custom("invalid number"))?;
            
            match T::try_from(n) {
                Ok(c) => {
                    Ok(Some(c))
                },
                Err(_e) => {
                    Err(de::Error::custom("number too large to fit in target type"))
                }
            }
        },
        Value::Null => Ok(None),
        _ => Err(de::Error::custom("not a number")),
    }
}

pub fn from_optional_float_or_string<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>
{
    match Value::deserialize(deserializer)? {
        Value::String(s) => {
            if s.is_empty() {
                return Ok(None);
            }
            
            let n = s.parse::<f64>().map_err(de::Error::custom)?;
                
            Ok(Some(n))
        },
        Value::Number(num) => {
            let n: f64 = num.as_f64().ok_or_else(|| de::Error::custom("invalid number"))?;
            
            Ok(Some(n))
        },
        Value::Null => Ok(None),
        _ => Err(de::Error::custom("not a number")),
    }
}

pub fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    let opt = Option::<String>::deserialize(de)?;
    let opt = opt.as_deref();
    
    match opt {
        None | Some("") => Ok(None),
        Some(s) => T::deserialize(s.into_deserializer()).map(Some)
    }
}

pub fn deserialize_attributes<'de, D>(deserializer: D) -> Result<Attributes, D::Error>
where
    D: Deserializer<'de>,
{
    struct ItemsVisitor;
    
    impl<'de> Visitor<'de> for ItemsVisitor {
        type Value = Attributes;
        
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence of items")
        }
        
        fn visit_seq<V>(self, mut seq: V) -> Result<Attributes, V::Error>
        where
            V: SeqAccess<'de>,
        {
            let mut map = Attributes::with_capacity(seq.size_hint().unwrap_or(0));
            
            while let Some(item) = seq.next_element::<Attribute>()? {
                map.insert(item.defindex, item);
            }
            
            Ok(map)
        }
    }
    
    deserializer.deserialize_seq(ItemsVisitor)
}

pub fn deserialize_listing_bans<'de, D>(deserializer: D) -> Result<Vec<Ban>, D::Error>
where
    D: Deserializer<'de>,
{
    struct BansVisitor;
    
    impl<'de> Visitor<'de> for BansVisitor {
        type Value = Vec<Ban>;
        
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence or map")
        }
        
        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
        where
            V: SeqAccess<'de>,
        {
            let mut bans = Vec::with_capacity(seq.size_hint().unwrap_or(0));
            
            while let Some(ban) = seq.next_element::<Ban>()? {
                bans.push(ban);
            }
            
            Ok(bans)
        }
        
        fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut bans = Vec::new();
            
            // key is an eum for the ban type, value is ban duration as a unix timestamp
            while let Some((_ban_type, _timestamp)) = access.next_entry::<u32, u32>()? {
                bans.push(Ban {});
            }
            
            Ok(bans)
        }
    }
    
    deserializer.deserialize_any(BansVisitor)
}

pub fn listing_intent_enum_from_str<'de, D>(deserializer: D) -> Result<ListingIntent, D::Error>
where
    D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    
    match s.as_str() {
        "buy" => Ok(ListingIntent::Buy),
        "sell" => Ok(ListingIntent::Sell),
        _ => Err(de::Error::custom("invalid intent")),
    }
}

pub fn currency_type_enum_from_str<'de, D>(deserializer: D) -> Result<CurrencyType, D::Error>
where
    D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    
    match s.as_str() {
        "keys" => Ok(CurrencyType::Keys),
        "metal" => Ok(CurrencyType::Metal),
        _ => Err(de::Error::custom("invalid currency type")),
    }
}

pub fn default_on_null<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    Ok(Option::deserialize(deserializer)?.unwrap_or_default())
}

pub fn string_or_number<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + TryFrom<i64> + TryFrom<u64> + Deserialize<'de>,
    T::Err: Display,
{
    struct NumericVisitor<T> {
        marker: PhantomData<T>,
    }
    
    impl<T> NumericVisitor<T> {
        pub fn new() -> Self {
            Self {
                marker: PhantomData,
            }
        }
    }
    
    impl<'de, T> de::Visitor<'de> for NumericVisitor<T>
    where 
        T: FromStr + TryFrom<i64> + TryFrom<u64> + Deserialize<'de>,
        T::Err: Display,
    {
        type Value = T;
    
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an integer or a string")
        }
    
        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match T::try_from(v) {
                Ok(c) => {
                    Ok(c)
                },
                Err(_e) => {
                    Err(de::Error::custom("number too large to fit in target type"))
                }
            }
        }
    
        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match T::try_from(v) {
                Ok(c) => {
                    Ok(c)
                },
                Err(_e) => {
                    Err(de::Error::custom("number too large to fit in target type"))
                }
            }
        }
    
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            v.parse::<T>().map_err(de::Error::custom)
        }
    }
    
    deserializer.deserialize_any(NumericVisitor::new())
}