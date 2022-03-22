use serde::Serializer;
use crate::{ListingIntent, CurrencyType};
use steamid_ng::SteamID;
use crate::response::attributes::FloatValue;

fn comma_delimited_values<T: ToString>(values: &[T]) -> String {
    values
        .iter()
        .map(|value| value.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

pub fn as_string<S, T>(value: &T, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: ToString,
{
    s.serialize_str(&value.to_string())
}

pub fn comma_delimited<S, T>(values: &[T], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: ToString,
{
    s.serialize_str(&comma_delimited_values(values))
}

pub fn option_number_to_str<S, T>(value: &Option<T>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: ToString,
{
    if let Some(ref v) = *value {
        s.serialize_str(&v.to_string())
    } else {
        s.serialize_none()
    }
}

pub fn comma_delimited_steamids<S>(values: &[SteamID], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    let steamids = values
        .iter()
        .map(|steamid| u64::from(*steamid).to_string())
        .collect::<Vec<String>>()
        .join(",");
    
    s.serialize_str(&steamids)
}

// todo make this generic
pub fn listing_intent_enum_to_str<S>(value: &ListingIntent, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    s.serialize_str(match *value {
        ListingIntent::Buy => "buy",
        ListingIntent::Sell => "sell",
    })
}

pub fn currency_type_enum_to_str<S>(value: &Option<CurrencyType>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    if let Some(currency_type) = value {
        s.serialize_str(match currency_type {
            CurrencyType::Keys => "keys",
            CurrencyType::Metal => "metal",
        })
    } else {
        s.serialize_none()
    }
}

pub fn option_float_as_integers_when_whole<S>(value: &Option<FloatValue>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(value) = value {
        if value.fract() == 0.0 {
            s.serialize_u64(*value as u64)
        } else {
            s.serialize_f64(*value)
        }
    } else {
        s.serialize_none()
    }
}