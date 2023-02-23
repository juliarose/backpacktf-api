use super::{Attributable, ItemAttribute};
use crate::response::attributes::{Value as AttributeValue, FloatValue};
use serde::{Deserialize, Serialize};
use tf2_enum::{KillstreakTier, Wear, Quality};

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Copy)]
pub struct Item {
    pub defindex: u32,
    pub quality: Quality,
    pub craftable: bool,
    pub killstreak_tier: Option<KillstreakTier>,
    pub particle: Option<u32>,
    pub wear: Option<Wear>,
    pub skin: Option<u32>,
    pub strange: bool,
    pub festivized: bool,
    pub australium: bool,
}

impl Item {
    pub fn new(defindex: u32, quality: Quality) -> Self {
        Self {
            defindex,
            quality,
            craftable: true,
            killstreak_tier: None,
            particle: None,
            wear: None,
            skin: None,
            strange: false,
            festivized: false,
            australium: false,
        }
    }
}

impl Default for Item {
    fn default() -> Self {
        Self {
            defindex: 0,
            quality: Quality::Unique,
            craftable: true,
            killstreak_tier: None,
            particle: None,
            wear: None,
            skin: None,
            strange: false,
            festivized: false,
            australium: false,
        }
    }
}

impl Attributable for Item {
    fn as_attributes(&self) -> Vec<ItemAttribute> {
        let mut attributes: Vec<ItemAttribute> = Vec::new();
        
        if let Some(killstreak_tier) = &self.killstreak_tier {
            
            attributes.push(ItemAttribute {
                // killstreak_tier
                defindex: 2025,
                value: None,
                float_value: Some(*killstreak_tier as u32 as f64),
            });
        }
        
        if let Some(particle) = self.particle {
            attributes.push(ItemAttribute {
                // set_attached_particle
                defindex: 134,
                value: None,
                float_value: Some(particle as f64),
            });
        }
        
        if self.strange {
            attributes.push(ItemAttribute {
                // kill_eater
                defindex: 214,
                value: None,
                float_value: None,
            });
        }
        
        if self.australium {
            attributes.push(ItemAttribute {
                // is_australium_item
                defindex: 2027,
                value: None,
                float_value: None,
            });
        }
        
        if self.festivized {
            attributes.push(ItemAttribute {
                // is_festive
                defindex: 2053,
                value: None,
                float_value: None,
            });
        }
        
        if let Some(wear) = &self.wear {
            let float_value: FloatValue = match wear {
                Wear::FactoryNew => 0.2,
                Wear::MinimalWear => 0.4,
                Wear::FieldTested => 0.6,
                Wear::WellWorn => 0.8,
                Wear::BattleScarred => 1.0,
            };
            
            attributes.push(ItemAttribute {
                // set_item_texture_wear
                defindex: 725,
                value: None,
                float_value: Some(float_value),
            });
        }
        
        if let Some(skin) = self.skin {
            attributes.push(ItemAttribute {
                // paintkit_proto_def_index
                defindex: 834,
                value: Some(AttributeValue::Number(skin as u64)),
                float_value: None,
            });
        }
        
        attributes
    }
}