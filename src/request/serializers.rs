use serde::{Serializer};
use crate::ListingIntent;
use tf2_price::{Currencies, CurrenciesForm};
use steamid_ng::SteamID;

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

pub fn currencies_into_form<S>(currencies: &Currencies, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    let form: CurrenciesForm = currencies.clone().into();
    
    s.serialize_newtype_struct("CurrenciesForm", &form)
}

pub fn listing_intent_enum_to_str<S>(value: &ListingIntent, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    s.serialize_str(match *value {
        ListingIntent::Buy => "buy",
        ListingIntent::Sell => "sell",
    })
}
