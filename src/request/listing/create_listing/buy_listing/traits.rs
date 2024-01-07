use super::ItemAttribute;

/// A trait for types that can be converted to a list of attributes.
pub trait Attributable {
    fn as_attributes(&self) -> Vec<ItemAttribute>;
}
