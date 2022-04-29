use serde::{Serialize, Deserialize};
use crate::SteamID;
use crate::response::attributes::{Attributes, Value as AttributeValue};
use crate::response::deserializers::{
    deserialize_attributes,
    from_optional_number_or_string
};
use tf2_enum::{
    Wear, Quality, KillstreakTier, Paint, StrangePart, Killstreaker, Sheen, Origin,
    Spell, FootprintsSpell, PaintSpell, Attribute, Attributes as EnumAttributes
};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Item {
    pub defindex: u32,
    pub quality: Quality,
    #[serde(default)]
    pub flag_cannot_craft: bool,
    #[serde(default)]
    #[serde(deserialize_with = "from_optional_number_or_string")]
    pub id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "from_optional_number_or_string")]
    pub original_id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "from_optional_number_or_string")]
    pub level: Option<u8>,
    #[serde(default)]
    #[serde(deserialize_with = "from_optional_number_or_string")]
    pub inventory: Option<u32>,
    #[serde(default)]
    #[serde(deserialize_with = "from_optional_number_or_string")]
    pub quantity: Option<u32>,
    #[serde(default)]
    pub origin: Option<Origin>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_attributes")]
    pub attributes: Attributes,
    pub marketplace_price: Option<f32>,
    pub marketplace_bot_steamid: Option<SteamID>,
    pub marketplace_sku: Option<String>,
    pub marketplace_image: Option<String>,
}

fn convert_float_u32(float: f64) -> Option<u32> {
    let int = float as u32;
    
    if int as f64 == float {
        Some(int)
    } else {
        None
    }
}

impl Item {

    pub fn get_quality(&self) -> Quality {
        self.quality.clone()
    }
    
    // todo - I may change these to return explicit errors later on
    // (attribute does not exist, failed to parse attribute)
    // in addition, most of these methods can be written in a more generic way
    pub fn get_particle_value(&self) -> Option<u32> {
        if let Some(attribute) = self.attributes.get(&134) {
            if let Some(float_value) = attribute.float_value {
                return convert_float_u32(float_value);
            }
        }
        
        None
    }
    
    pub fn get_skin_value(&self) -> Option<u32> {
        if let Some(attribute) = self.attributes.get(&834) {
            if let Some(AttributeValue::Number(value)) = attribute.value {
                if let Ok(value) = u32::try_from(value) {
                    return Some(value);
                }
            }
        }
        
        None
    }
    
    pub fn get_killstreak_tier(&self) -> Option<KillstreakTier> {
        if let Some(attribute) = self.attributes.get(&(KillstreakTier::DEFINDEX as i32)) {
            if let Some(float_value) = attribute.float_value {
                if let Some(float_value) = convert_float_u32(float_value) {
                    if let Ok(killstreak_tier) = KillstreakTier::try_from(float_value) {
                        return Some(killstreak_tier);
                    }
                }
            }
        }
        
        None
    }
    
    pub fn get_wear(&self) -> Option<Wear> {
        if let Some(attribute) = self.attributes.get(&(Wear::DEFINDEX as i32)) {
            if let Some(float_value) = attribute.float_value {
                if let Ok(wear) = Wear::try_from(float_value) {
                    return Some(wear);
                }
            }
        }
        
        None
    }
    
    pub fn get_spells(&self) -> Option<Vec<Spell>> {
        let spells = Spell::DEFINDEX
            .iter()
            .filter_map(|defindex| {
                if let Some(attribute) = self.attributes.get(&(*defindex as i32)) {
                    match *defindex {
                        Spell::DEFINDEX_FOOTPRINTS => {
                            if let Some(float_value) = attribute.float_value {
                                if let Some(float_value) = convert_float_u32(float_value) {
                                    if let Ok(spell) = FootprintsSpell::try_from(float_value) {
                                        return Some(Spell::Footprints(spell));
                                    }
                                }
                            }
                            
                            None
                        },
                        Spell::DEFINDEX_PAINT => {
                            if let Some(float_value) = attribute.float_value {
                                if let Some(float_value) = convert_float_u32(float_value) {
                                    if let Ok(spell) = PaintSpell::try_from(float_value) {
                                        return Some(Spell::Paint(spell));
                                    }
                                }
                            }
                            
                            None
                        },
                        Spell::DEFINDEX_VOICES_FROM_BELOW => Some(Spell::VoicesFromBelow),
                        Spell::DEFINDEX_PUMPKIN_BOMBS => Some(Spell::PumpkinBombs),
                        Spell::DEFINDEX_HALLOWEEN_FIRE => Some(Spell::HalloweenFire),
                        Spell::DEFINDEX_EXORCISM => Some(Spell::Exorcism),
                        _ => None,
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<Spell>>();
        
        if !spells.is_empty() {
            Some(spells)
        } else {
            None
        }
    }
    
    pub fn get_strange_parts(&self) -> Option<Vec<StrangePart>> {
        let strange_parts = StrangePart::DEFINDEX 
            .iter()
            .filter_map(|defindex| {
                self.attributes.get(&(*defindex as i32))
                    .and_then(|attribute| attribute.float_value)
                    .and_then(|float_value| {
                        if let Some(float_value) = convert_float_u32(float_value) {
                            if let Ok(strange_part) = StrangePart::try_from(float_value) {
                                return Some(strange_part);
                            }
                        }
                        
                        None
                    })
            })
            .collect::<Vec<StrangePart>>();
        
        if !strange_parts.is_empty() {
            Some(strange_parts)
        } else {
            None
        }
    }
    
    pub fn get_paint(&self) -> Option<Paint> {
        if self.defindex < 5027 || self.defindex > 5077 {
            if let Some(attribute) = self.attributes.get(&(Paint::DEFINDEX as i32)) {
                if let Some(float_value) = attribute.float_value {
                    if let Some(float_value) = convert_float_u32(float_value) {
                        if let Ok(paint) = Paint::try_from(float_value) {
                            return Some(paint);
                        }
                    }
                }
            }
        }
        
        None
    }
    
    pub fn get_killstreaker(&self) -> Option<Killstreaker> {
        if let Some(attribute) = self.attributes.get(&(Killstreaker::DEFINDEX as i32)) {
            if let Some(float_value) = attribute.float_value {
                if let Some(float_value) = convert_float_u32(float_value) {
                    if let Ok(killstreaker) = Killstreaker::try_from(float_value) {
                        return Some(killstreaker);
                    }
                }
            }
        }
        
        None
    }
    
    pub fn get_sheen(&self) -> Option<Sheen> {
        if let Some(attribute) = self.attributes.get(&(Sheen::DEFINDEX as i32)) {
            if let Some(float_value) = attribute.float_value {
                if let Some(float_value) = convert_float_u32(float_value) {
                    if let Ok(sheen) = Sheen::try_from(float_value) {
                        return Some(sheen);
                    }
                }
            }
        }
        
        None
    }
    
    pub fn is_craftable(&self) -> bool {
        !self.flag_cannot_craft
    }
    
    pub fn is_australium(&self) -> bool {
        self.attributes.contains_key(&2027)
    }
    
    pub fn is_festivized(&self) -> bool {
        self.attributes.contains_key(&2053)
    }
    
    pub fn is_strange(&self) -> bool {
        // strange quality items are not "strangified" items
        if self.quality == Quality::Strange {
            false
        } else {
            self.attributes.contains_key(&214)
        }
    }
}