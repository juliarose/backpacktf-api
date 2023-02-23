use std::fmt::Display;
use serde::ser::Serializer;

pub fn to_display<T, S>(
    item: T,
    serializer: S,
) -> Result<S::Ok, S::Error> where T: Display, S: Serializer {
    serializer.serialize_str(&item.to_string())
}