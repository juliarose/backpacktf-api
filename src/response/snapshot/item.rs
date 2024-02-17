//! Snapshot item.

use crate::SteamID;
use crate::response::attributes::{Attributes, Value as AttributeValue};
use crate::response::deserializers;
use serde::{Serialize, Deserialize};
use tf2_enum::{
    Wear, Quality, KillstreakTier, Paint, StrangePart, Killstreaker, Sheen, Origin,
    Spell, FootprintsSpell, PaintSpell, Attribute, Attributes as EnumAttributes,
};

/// An item in a snapshot.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Item {
    // must be i32 to account for marketplace cross-listing SKUs such as -100 (Random Craft Hat)
    /// The item's defindex. This is not necessarily mapped to an item in the schema and can 
    /// include values for non-standard items such as "Random Craft Hat" (-100).
    pub defindex: i32,
    /// The quality of the item.
    pub quality: Quality,
    /// Whether the is craftable or not.
    #[serde(default)]
    pub flag_cannot_craft: bool,
    /// The ID of the item.
    #[serde(default)]
    #[serde(deserialize_with = "deserializers::from_optional_number_or_string")]
    pub id: Option<u64>,
    /// The original ID of the item.
    #[serde(default)]
    #[serde(deserialize_with = "deserializers::from_optional_number_or_string")]
    pub original_id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserializers::from_optional_number_or_string_integer")]
    // Levels can sometimes be negative.
    /// The level of the item.
    pub level: Option<i32>,
    /// The position of the item in the backpack.
    #[serde(default)]
    #[serde(deserialize_with = "deserializers::from_optional_number_or_string")]
    pub inventory: Option<u32>,
    /// The quantity of the item.
    #[serde(default)]
    #[serde(deserialize_with = "deserializers::from_optional_number_or_string")]
    pub quantity: Option<u32>,
    /// The item's origin.
    #[serde(default)]
    pub origin: Option<Origin>,
    /// The item's attributes.
    #[serde(default)]
    #[serde(deserialize_with = "deserializers::deserialize_attributes")]
    pub attributes: Attributes,
    /// The item's marketplace price.
    pub marketplace_price: Option<f32>,
    /// The marketplace bot's SteamID if the item is listed on marketplace.tf.
    pub marketplace_bot_steamid: Option<SteamID>,
    /// The marketplace SKU if the item is listed on marketplace.tf.
    pub marketplace_sku: Option<String>,
    /// The marketplace image URL if the item is listed on marketplace.tf.
    pub marketplace_image: Option<String>,
}

impl Item {
    /// Gets the quality of the item.
    pub fn get_quality(&self) -> Quality {
        self.quality
    }
    
    /// Gets the particle value of the item.
    pub fn get_particle_value(&self) -> Option<u32> {
        get_attributes_float_u32(
            &self.attributes,
            134
        )
    }
    
    /// Gets the skin value of the item.
    pub fn get_skin_value(&self) -> Option<u32> {
        let attribute = self.attributes.get(&834)?;
        
        if let Some(AttributeValue::Number(value)) = attribute.value {
            let value = u32::try_from(value).ok()?;
            
            return Some(value);
        }
        
        None
    }
    
    /// Gets the killstreak tier of the item.
    pub fn get_killstreak_tier(&self) -> Option<KillstreakTier> {
        try_from_float_value(&self.attributes)
    }
    
    /// Gets the wear of the item.
    pub fn get_wear(&self) -> Option<Wear> {
        let attribute = self.attributes.get(&(Wear::DEFINDEX as i32))?;
        let float_value = attribute.float_value?;
        
        Wear::try_from(float_value).ok()
    }
    
    /// Gets the spells on the item.
    pub fn get_spells(&self) -> Option<Vec<Spell>> {
        let spells = Spell::DEFINDEX
            .iter()
            .filter_map(|defindex| {
                let attribute = self.attributes.get(&(*defindex as i32))?;
                
                match *defindex {
                    Spell::DEFINDEX_FOOTPRINTS => {
                        let float_value = attribute.float_value?;
                        let float_value = convert_float_u32(float_value)?;
                        
                        FootprintsSpell::try_from(float_value).ok()
                            .map(|spell| spell.into())
                    },
                    Spell::DEFINDEX_PAINT => {
                        let float_value = attribute.float_value?;
                        let float_value = convert_float_u32(float_value)?;
                        
                        PaintSpell::try_from(float_value).ok()
                            .map(|spell| spell.into())
                    },
                    Spell::DEFINDEX_VOICES_FROM_BELOW => Some(Spell::VoicesFromBelow),
                    Spell::DEFINDEX_PUMPKIN_BOMBS => Some(Spell::PumpkinBombs),
                    Spell::DEFINDEX_HALLOWEEN_FIRE => Some(Spell::HalloweenFire),
                    Spell::DEFINDEX_EXORCISM => Some(Spell::Exorcism),
                    _ => None,
                }
            })
            .collect::<Vec<Spell>>();
        
        if !spells.is_empty() {
            Some(spells)
        } else {
            None
        }
    }
    
    /// Gets the strange parts on the item.
    pub fn get_strange_parts(&self) -> Option<Vec<StrangePart>> {
        let strange_parts = StrangePart::DEFINDEX 
            .iter()
            .filter_map(|defindex| {
                let defindex = i32::try_from(*defindex).ok()?;
                let float_value = get_attributes_float_u32(
                    &self.attributes,
                    defindex,
                )?;
                
                StrangePart::try_from(float_value).ok()
            })
            .collect::<Vec<StrangePart>>();
        
        if !strange_parts.is_empty() {
            Some(strange_parts)
        } else {
            None
        }
    }
    
    /// Gets the paint on the item.
    pub fn get_paint(&self) -> Option<Paint> {
        // 5027-5077 are paint defindexes
        if self.defindex >= 5027 || self.defindex <= 5077 {
            return None;
        }
        
        try_from_float_value(&self.attributes)
    }
    
    /// Gets the killstreaker of the item.
    pub fn get_killstreaker(&self) -> Option<Killstreaker> {
        try_from_float_value(&self.attributes)
    }
    
    /// Gets the sheen of the item.
    pub fn get_sheen(&self) -> Option<Sheen> {
        try_from_float_value(&self.attributes)
    }
    
    /// Checks if the item is craftable.
    pub fn is_craftable(&self) -> bool {
        !self.flag_cannot_craft
    }
    
    /// Checks if the item is australium.
    pub fn is_australium(&self) -> bool {
        self.attributes.contains_key(&2027)
    }
    
    /// Checks if the item is festivized.
    pub fn is_festivized(&self) -> bool {
        self.attributes.contains_key(&2053)
    }
    
    /// Checks if the item is strange.
    pub fn is_strange(&self) -> bool {
        // strange quality items are not "strangified" items
        if self.quality == Quality::Strange {
            false
        } else {
            self.attributes.contains_key(&214)
        }
    }
}

fn convert_float_u32(float: f64) -> Option<u32> {
    let int = float as u32;
    
    if int as f64 == float {
        Some(int)
    } else {
        None
    }
}

/// Gets the float value of an attribute as a u32.
fn get_attributes_float_u32(
    attributes: &Attributes,
    defindex: i32,
) -> Option<u32> {
    let attribute = attributes.get(&defindex)?;
    let float_value = attribute.float_value?;
    
    convert_float_u32(float_value)
}

/// Attempts to convert an attribute float value to an enum value.
fn try_from_float_value<T>(
    attributes: &Attributes,
) -> Option<T>
where
    T: TryFrom<u32> + Attribute,
{
    let defindex = i32::try_from(T::DEFINDEX).ok()?;
    let float_value = get_attributes_float_u32(
        attributes,
        defindex,
    )?;
    
    T::try_from(float_value).ok()
}
