use super::ItemAttribute;

pub trait Attributable {
    fn as_attributes(&self) -> Vec<ItemAttribute>;
}
