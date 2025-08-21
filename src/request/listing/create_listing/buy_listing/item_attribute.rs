use crate::response::attributes::{Value, FloatValue};
use crate::request::serializers::option_float_as_integers_when_whole;
use serde::{Deserialize, Serialize};

/// An item attribute. For a list of attributes see the 
/// [wiki](https://wiki.teamfortress.com/wiki/List_of_item_attributes).
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct ItemAttribute {
    /// The defindex of the attribute.
    pub defindex: i32,
    /// The value of the attribute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    /// The float value of the attribute.
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "option_float_as_integers_when_whole")]
    pub float_value: Option<FloatValue>,
}

impl ItemAttribute {
    /// Creates a new attribute with the given defindex and value.
    pub(crate) fn from_attribute<A>(attribute: &A) -> Self
    where
        A: tf2_enum::Attribute,
    {
        // Only serialize based on which value is meaningful.
        // The other value is the same number as a float which the backpack.tf API doesn't need.
        let (value, float_value) = pick_value_float(
            A::USES_FLOAT_VALUE,
            || attribute.attribute_value(),
            || attribute.attribute_float_value(),
        );
        
        Self {
            defindex: A::DEFINDEX as i32,
            value,
            float_value,
        }
    }
    
    /// Creates a new attribute with the given defindex and value.
    pub(crate) fn from_attributes<A>(attribute: tf2_enum::ItemAttribute) -> Self
    where
        A: tf2_enum::Attributes,
    {
        let (value, float_value) = pick_value_float(
            A::USES_FLOAT_VALUE,
            || attribute.value,
            || attribute.float_value,
        );
        
        Self {
            defindex: attribute.defindex as i32,
            value,
            float_value,
        }
    }
    
    pub(crate) fn from_attributes_with_defindex<A>(
        attribute: &A,
        defindex: i32,
    ) -> Self
    where
        A: tf2_enum::Attributes,
    {
        let (value, float_value) = pick_value_float(
            A::USES_FLOAT_VALUE,
            || attribute.attribute_value(),
            || attribute.attribute_float_value(),
        );
        
        Self {
            defindex,
            value,
            float_value,
        }
    }
}

// Picks the appropriate value and float value based on the bool.
fn pick_value_float<F, G>(
    uses_float: bool,
    get_value: F,
    get_float: G,
) -> (Option<Value>, Option<FloatValue>)
where
    F: FnOnce() -> tf2_enum::AttributeValue,
    G: FnOnce() -> Option<f32>,
{
    if uses_float {
        (None, Some(get_float().unwrap_or(0.0)))
    } else {
        (Some(get_value().into()), None)
    }
}

impl From<tf2_enum::ItemAttribute> for ItemAttribute {
    fn from(attribute: tf2_enum::ItemAttribute) -> Self {
        Self {
            defindex: attribute.defindex as i32,
            value: Some(attribute.value.into()),
            float_value: attribute.float_value.map(FloatValue::from),
        }
    }
}
