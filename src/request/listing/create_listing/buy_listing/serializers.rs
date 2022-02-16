use serde::Serializer;
use super::{item::Item, item_params::ItemParams};
use crate::response::attributes::FloatValue;

pub fn buy_listing_item_into_params<S>(query: &Item, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    let form: ItemParams = query.into();
    
    s.serialize_newtype_struct("ItemParams", &form)
}

pub fn option_buy_listing_item_into_params<S>(value: &Option<&Item>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    if let Some(query) = *value {
        let form: ItemParams = query.into();
        
        s.serialize_newtype_struct("ItemParams", &form)
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