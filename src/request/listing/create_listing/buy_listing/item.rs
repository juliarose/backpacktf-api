use super::{Attributable, ItemAttribute};
use serde::{Deserialize, Serialize};
use tf2_enum::{Attributes, AttributeSet, KillstreakTier, Paint, Quality, SpellSet, StrangePartSet, Wear};
use tf2_enum::econ_attributes::{
    IsAustralium, IsFestivized, KillEater, KillEaterUserScore, PaintkitProtoDefIndex, SetAttachedParticle
};

/// An item.
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Copy)]
pub struct Item {
    /// The defindex of the item.
    pub defindex: u32,
    /// The quality of the item.
    pub quality: Quality,
    /// Whether the item is craftable.
    pub craftable: bool,
    /// The killstreak tier of the item.
    pub killstreak_tier: Option<KillstreakTier>,
    /// The particle of the item.
    pub particle: Option<u32>,
    /// The wear of the item.
    pub wear: Option<Wear>,
    /// The skin of the item.
    pub skin: Option<u32>,
    /// Whether the item is strange.
    pub strange: bool,
    /// Whether the item is festivized.
    pub festivized: bool,
    /// Whether the item is australium.
    pub australium: bool,
    /// The paint of the item.
    pub paint: Option<Paint>,
    /// The spells of the item.
    #[serde(skip_serializing_if = "SpellSet::is_empty")]
    pub spells: SpellSet,
    /// The strange parts of the item.
    #[serde(skip_serializing_if = "StrangePartSet::is_empty")]
    pub strange_parts: StrangePartSet,
}

impl Item {
    /// Creates a new item with the given defindex and quality.
    pub fn new(
        defindex: u32,
        quality: Quality,
    ) -> Self {
        Self {
            defindex,
            quality,
            ..Default::default()
        }
    }
    
    /// Sets the defindex.
    pub fn defindex(mut self, defindex: u32) -> Self {
        self.defindex = defindex;
        self
    }
    
    /// Sets the quality.
    pub fn quality(mut self, quality: Quality) -> Self {
        self.quality = quality;
        self
    }

    /// Sets whether the item is craftable.
    pub fn craftable(mut self, craftable: bool) -> Self {
        self.craftable = craftable;
        self
    }

    /// Sets the killstreak tier.
    pub fn killstreak_tier(mut self, killstreak_tier: KillstreakTier) -> Self {
        self.killstreak_tier = Some(killstreak_tier);
        self
    }

    /// Sets the particle.
    pub fn particle(mut self, particle: u32) -> Self {
        self.particle = Some(particle);
        self
    }

    /// Sets the wear.
    pub fn wear(mut self, wear: Option<Wear>) -> Self {
        self.wear = wear;
        self
    }

    /// Sets the skin.
    pub fn skin(mut self, skin: u32) -> Self {
        self.skin = Some(skin);
        self
    }

    /// Sets whether the item is strange.
    pub fn strange(mut self, strange: bool) -> Self {
        self.strange = strange;
        self
    }

    /// Sets whether the item is festivized.
    pub fn festivized(mut self, festivized: bool) -> Self {
        self.festivized = festivized;
        self
    }

    /// Sets whether the item is australium.
    pub fn australium(mut self, australium: bool) -> Self {
        self.australium = australium;
        self
    }

    /// Sets the paint.
    pub fn paint(mut self, paint: Paint) -> Self {
        self.paint = Some(paint);
        self
    }
    
    /// Sets the spells.
    pub fn spells(mut self, spells: SpellSet) -> Self {
        self.spells = spells;
        self
    }
    
    /// Sets the strange parts.
    pub fn strange_parts(mut self, strange_parts: StrangePartSet) -> Self {
        self.strange_parts = strange_parts;
        self
    }
    
    fn compute_attribute_len(&self) -> usize {
        self.killstreak_tier.is_some() as usize
            + self.particle.is_some() as usize
            + self.strange as usize
            + self.australium as usize
            + self.festivized as usize
            + self.paint.is_some() as usize
            + self.wear.is_some() as usize
            + self.skin.is_some() as usize
            + self.spells.len()
            + (self.strange_parts.len() * 2)
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
            paint: None,
            spells: SpellSet::default(),
            strange_parts: StrangePartSet::default(),
        }
    }
}

impl Attributable for Item {
    fn as_attributes(&self) -> Vec<ItemAttribute> {
        // Since we know exactly how many elements we need to insert, we can preallocate the
        // vector.
        let capacity = self.compute_attribute_len();
        let mut attributes: Vec<ItemAttribute> = Vec::with_capacity(capacity);
        
        if let Some(killstreak_tier) = &self.killstreak_tier {
            attributes.push(as_attr(killstreak_tier));
        }
        
        if let Some(particle) = self.particle {
            attributes.push(as_attr(&SetAttachedParticle::from(particle)));
        }
        
        if self.strange {
            attributes.push(as_attr(&KillEater::default()));
        }
        
        if self.australium {
            attributes.push(as_attr(&IsAustralium::default()));
        }
        
        if self.festivized {
            attributes.push(as_attr(&IsFestivized::default()));
        }
        
        if let Some(paint) = &self.paint {
            attributes.push(as_attr(paint));
        }
        
        if let Some(wear) = &self.wear {
            attributes.push(as_attr(wear));
        }
        
        if let Some(skin) = self.skin {
            attributes.push(as_attr(&PaintkitProtoDefIndex::from(skin as u32)));
        }
        
        for attribute in self.spells.iter_attributes() {
            attributes.push(ItemAttribute::from_attributes::<<SpellSet as AttributeSet>::Item>(attribute));
        }
        
        for (
            attribute,
            defindex,
        ) in self.strange_parts.iter_attributes().zip(KillEaterUserScore::DEFINDEX) {
            // Probably not entirely necessary but also push the attribute for the count.
            attributes.push(ItemAttribute::from_attributes_with_defindex(
                &KillEaterUserScore(100),
                *defindex as i32,
            ));
            attributes.push(ItemAttribute::from_attributes::<<StrangePartSet as AttributeSet>::Item>(attribute));
        }
        
        attributes
    }
}


fn as_attr<A>(item: &A) -> ItemAttribute
where
    A: tf2_enum::Attribute
{
    ItemAttribute::from_attribute(item)
}
