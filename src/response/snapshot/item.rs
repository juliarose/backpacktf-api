use serde::{Serialize, Deserialize};
use crate::response::attributes::{Attributes, Value as AttributeValue};
use crate::response::deserializers::{
    deserialize_attributes,
    from_optional_number_or_string
};
use tf2_enum::{Wear, Quality, KillstreakTier};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Item {
    pub defindex: u32,
    pub quality: Quality,
    #[serde(default)]
    #[serde(deserialize_with = "from_optional_number_or_string")]
    pub id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "from_optional_number_or_string")]
    pub original_id: Option<u64>,
    pub level: Option<u8>,
    pub inventory: Option<u32>,
    #[serde(default)]
    #[serde(deserialize_with = "from_optional_number_or_string")]
    pub quantity: Option<u32>,
    pub origin: Option<u32>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_attributes")]
    pub attributes: Attributes,
}

impl Item {

    pub fn get_quality(&self) -> Quality {
        self.quality.clone()
    }
    
    pub fn get_particle_value(&self) -> Option<u32> {
        if let Some(attribute) = self.attributes.get(&134) {
            if let Some(float_value) = &attribute.float_value {
                return Some(*float_value as u32);
            }
        }
        
        None
    }
    
    pub fn get_skin_value(&self) -> Option<u32> {
        if let Some(attribute) = self.attributes.get(&834) {
            if let Some(value) = &attribute.value {
                if let AttributeValue::Number(value) = value {
                    return Some(*value as u32);
                }
            }
        }
        
        None
    }
    
    pub fn get_killstreak_tier(&self) -> Option<KillstreakTier> {
        if let Some(attribute) = self.attributes.get(&2025) {
            if let Some(float_value) = &attribute.float_value {
                if let Ok(killstreak_tier) = KillstreakTier::try_from(*float_value as u8) {
                    return Some(killstreak_tier);
                }
            }
        }
        
        None
    }
    
    pub fn get_wear(&self) -> Option<Wear> {
        if let Some(attribute) = self.attributes.get(&725) {
            if let Some(float_value) = &attribute.float_value {
                let wear_value = (float_value * 5.0).round() as u8;
                
                if let Ok(wear) = Wear::try_from(wear_value) {
                    return Some(wear);
                }
            }
        }
        
        None
    }
    
    pub fn is_australium(&self) -> bool {
        self.attributes.contains_key(&2027)
    }
    
    pub fn is_festive(&self) -> bool {
        self.attributes.contains_key(&2053)
    }
}