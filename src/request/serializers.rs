//! Serializers for request parameters.

use crate::{SteamID, ListingIntent, CurrencyType};
use crate::response::attributes::FloatValue;
use serde::Serializer;

/// Serializes into a string.
pub fn as_string<S, T>(
    value: &T,
    s: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: ToString,
{
    s.serialize_str(&value.to_string())
}

/// Serializes optional into a string.
pub fn option_number_to_str<S, T>(
    value: &Option<T>,
    s: S,
) -> Result<S::Ok, S::Error>
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

/// Serializes a list of SteamIDs into a comma delimited string.
pub fn comma_delimited_steamids<S>(
    values: &[SteamID],
    s: S,
) -> Result<S::Ok, S::Error>
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

/// Serializes listing intent into a string.
pub fn listing_intent_enum_to_str<S>(
    value: &ListingIntent,
    s: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    s.serialize_str(match *value {
        ListingIntent::Buy => "buy",
        ListingIntent::Sell => "sell",
    })
}

/// Serializes currency type into a string.
pub fn currency_type_enum_to_str<S>(
    value: &Option<CurrencyType>,
    s: S,
) -> Result<S::Ok, S::Error>
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

/// Serializes a float into an integer when the float is a whole number.
pub fn option_float_as_integers_when_whole<S: Serializer>(
    value: &Option<FloatValue>,
    s: S,
) -> Result<S::Ok, S::Error> {
    match value {
        Some(v) if v.fract() == 0.0 && (*v as u32 as FloatValue == *v) => s.serialize_u64(*v as u64),
        Some(v) => s.serialize_f32(*v),
        None => s.serialize_none(),
    }
}