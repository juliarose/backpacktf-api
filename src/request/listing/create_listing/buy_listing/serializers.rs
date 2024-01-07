use super::item::Item;
use super::item_params::ItemParams;
use serde::Serializer;

/// Serializes an item into parameters.
pub fn buy_listing_item_into_params<S>(query: &Item, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    let form: ItemParams = query.into();
    
    s.serialize_newtype_struct("ItemParams", &form)
}

/// Serializes an optional item into parameters.
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