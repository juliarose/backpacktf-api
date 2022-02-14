use std::collections::HashMap;
use crate::response::attributes::{Attributes, Attribute};
use crate::{ListingIntent, CurrencyType};
use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, Unexpected, IntoDeserializer};
use std::str::FromStr;
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
                Ok(None)
            } else {
                let n = s.parse::<T>().map_err(de::Error::custom)?;
                
                Ok(Some(n))
            }
        },
        Value::Number(num) => {
            let n: u64 = num.as_u64().ok_or(de::Error::custom("Invalid number"))?;
            
            match T::try_from(n) {
                Ok(c) => {
                    Ok(Some(c))
                },
                Err(_e) => {
                    Err(de::Error::custom("Number too large to fit in target type"))
                }
            }
        },
        Value::Null => Ok(None),
        _ => Err(de::Error::custom("Not a number")),
    }
}

pub fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    let opt = Option::<String>::deserialize(de)?;
    let opt = opt.as_ref().map(String::as_str);

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
        type Value = HashMap<u32, Attribute>;
        
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence of items")
        }
        
        fn visit_seq<V>(self, mut seq: V) -> Result<Attributes, V::Error>
        where
            V: SeqAccess<'de>,
        {
            let mut map = HashMap::with_capacity(seq.size_hint().unwrap_or(0));
            
            while let Some(item) = seq.next_element::<Attribute>()? {
                map.insert(item.defindex, item);
            }
            
            Ok(map)
        }
    }
    
    deserializer.deserialize_seq(ItemsVisitor)
}

pub fn listing_intent_enum_from_str<'de, D>(deserializer: D) -> Result<ListingIntent, D::Error>
where
    D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    
    match s.as_str() {
        "buy" => Ok(ListingIntent::Buy),
        "sell" => Ok(ListingIntent::Sell),
        _ => Err(de::Error::custom("Invalid intent")),
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
        _ => Err(de::Error::custom("Invalid currency type")),
    }
}
