use serde::{Deserialize, Serialize};
use tf2_enums::{KillstreakTier, Wear, Quality};
use super::{Attributable, BuyListingItemAttribute};
use crate::response::attributes::AttributeValue;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Item {
    pub defindex: u32,
    pub quality: Quality,
    pub craftable: bool,
    pub killstreak_tier: Option<KillstreakTier>,
    pub particle: Option<u32>,
    pub wear: Option<Wear>,
    pub strange: bool,
    pub festivized: bool,
    pub skin: Option<u32>,
}

impl Attributable for Item {
    fn as_attributes(&self) -> Vec<BuyListingItemAttribute> {
        let mut attributes: Vec<BuyListingItemAttribute> = Vec::new();
        
        if let Some(killstreak_tier) = &self.killstreak_tier {
            let number: u8 = killstreak_tier.clone().into();
            
            attributes.push(BuyListingItemAttribute {
                // killstreak_tier
                defindex: 2025,
                value: None,
                float_value: Some(AttributeValue::Number(number as u64)),
            });
        }
        
        if let Some(particle) = self.particle {
            attributes.push(BuyListingItemAttribute {
                // set_attached_particle
                defindex: 134,
                value: None,
                float_value: Some(AttributeValue::Number(particle as u64)),
            });
        }
        
        if self.strange {
            attributes.push(BuyListingItemAttribute {
                // kill_eater
                defindex: 214,
                value: None,
                float_value: None,
            });
        }
        
        if self.festivized {
            attributes.push(BuyListingItemAttribute {
                // is_festive
                defindex: 2053,
                value: None,
                float_value: None,
            });
        }
        
        if let Some(wear) = &self.wear {
            let float_value: f64 = match wear {
                Wear::FactoryNew => 0.2,
                Wear::MinimalWear => 0.4,
                Wear::FieldTested => 0.6,
                Wear::WellWorn => 0.8,
                Wear::BattleScarred => 1.0,
            };
            
            attributes.push(BuyListingItemAttribute {
                // set_item_texture_wear
                defindex: 725,
                value: None,
                float_value: Some(AttributeValue::Float(float_value as f64)),
            });
        }
        
        if let Some(skin) = self.skin {
            attributes.push(BuyListingItemAttribute {
                // paintkit_proto_def_index
                defindex: 834,
                value: Some(AttributeValue::Number(skin as u64)),
                float_value: None,
            });
        }
        
        attributes
    }
}