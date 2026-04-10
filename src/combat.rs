//! # Combat
//! Structs for combat-related character data: weapons, armor, and talents.
//!
//! Health and AP cap are derived stats and not stored directly:
//! - `max_health = 2 * will + 4 * forte`
//! - `ap_cap = 2 * speed_quantity`
//!
//! Injury is stored as a level count. Death occurs when `injury >= forte_quantity`.

use serde::{Deserialize, Serialize};

use bitcode::{Decode, Encode};

/// The physical damage category of a weapon's attack.
#[derive(Encode, Decode, Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub enum DamageType {
    #[default]
    Physical,
    Fire,
    Cold,
    Lightning,
    Poison,
    Other(String),
}

impl std::fmt::Display for DamageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Physical => write!(f, "Physical"),
            Self::Fire => write!(f, "Fire"),
            Self::Cold => write!(f, "Cold"),
            Self::Lightning => write!(f, "Lightning"),
            Self::Poison => write!(f, "Poison"),
            Self::Other(s) => write!(f, "{s}"),
        }
    }
}

/// A weapon on a character sheet.
///
/// `base_damage` is the flat base before stat rolls and modifiers.
/// `per_success_bonus_pct` is the `+X%` per success over that appears on the weapon card.
/// `stat_modifier` is the name of the core stat added as flat damage (e.g. `"Power"`).
/// `skill_requirement` is an optional `"{Quality}{Quantity}"` string (e.g. `"B3"`).
#[derive(Encode, Decode, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Weapon {
    pub name: String,
    pub skill: String,
    pub skill_requirement: Option<String>,
    pub base_damage: isize,
    pub stat_modifier: String,
    pub damage_type: DamageType,
    /// Percentage bonus per success over evasion, stored as integer (10 = +10%).
    pub per_success_bonus_pct: isize,
    pub notes: String,
}

impl Weapon {
    #[must_use]
    pub fn new(name: String) -> Self {
        Self {
            name,
            skill: String::new(),
            skill_requirement: None,
            base_damage: 1,
            stat_modifier: "Power".to_string(),
            damage_type: DamageType::Physical,
            per_success_bonus_pct: 0,
            notes: String::new(),
        }
    }
}

impl Default for Weapon {
    fn default() -> Self {
        Self::new(String::new())
    }
}

/// Armor worn by a character.
///
/// Either `flat_reduction` or `pct_reduction` (stored as integer, e.g. 25 = -25%)
/// may be non-zero; both can apply simultaneously.
#[derive(Encode, Decode, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Armor {
    pub name: String,
    /// Flat damage reduction applied before percentage reductions.
    pub flat_reduction: isize,
    /// Percentage damage reduction (e.g. 25 = −25%). Applied after flat.
    pub pct_reduction: isize,
    pub notes: String,
}

impl Armor {
    #[must_use]
    pub fn new(name: String) -> Self {
        Self {
            name,
            flat_reduction: 0,
            pct_reduction: 0,
            notes: String::new(),
        }
    }
}

impl Default for Armor {
    fn default() -> Self {
        Self::new(String::new())
    }
}

/// A talent available to a character in combat.
#[derive(Encode, Decode, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Talent {
    pub name: String,
    pub ap_cost: usize,
    pub required_skill: String,
    pub description: String,
}

impl Talent {
    #[must_use]
    pub fn new(name: String) -> Self {
        Self {
            name,
            ap_cost: 1,
            required_skill: String::new(),
            description: String::new(),
        }
    }
}

impl Default for Talent {
    fn default() -> Self {
        Self::new(String::new())
    }
}

/// Computes a character's maximum Health.
///
/// Formula: `2 × will_quantity + 4 × forte_quantity`
#[must_use]
pub fn max_health(will_quantity: usize, forte_quantity: usize) -> usize {
    2 * will_quantity + 4 * forte_quantity
}

/// Computes a character's AP cap from their Speed quantity.
///
/// Formula: `2 × speed_quantity`
#[must_use]
pub fn ap_cap(speed_quantity: usize) -> usize {
    2 * speed_quantity
}
