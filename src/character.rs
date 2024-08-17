// character.rs
// All structs and enums relating to characters.

/* Structs and Enums */

use serde::{Deserialize, Serialize};

use bitcode::{Decode, Encode};

use crate::{Inspiration, Quirk};

/// A struct containing all info about a character.
#[derive(Encode, Decode, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Character {
    pub name: String,
    pub stock: String,
    pub stats: Vec<Stat>,
    pub skills: Vec<Stat>,
    pub finite_resources: Vec<Stat>,
    pub infinite_resources: Vec<Stat>,
    pub quirks: Vec<Quirk>,
    pub inspiration: Inspiration,
    pub argos: String,
    pub inventory: Vec<Item>,
}

impl Character {
    #[must_use]
    pub fn new(name: String) -> Character {
        Character {
            name,
            stock: "Stock".to_string(),
            stats: vec![
                Stat::new("Will".into()),
                Stat::new("Perception".into()),
                Stat::new("Conscious".into()),
                Stat::new("Power".into()),
                Stat::new("Speed".into()),
                Stat::new("Forte".into()),
            ],
            skills: Vec::new(),
            quirks: Vec::new(),
            finite_resources: Vec::new(),
            infinite_resources: Vec::new(),
            inspiration: Inspiration::new(),
            argos: String::new(),
            inventory: Vec::new(),
        }
    }
}

impl Default for Character {
    fn default() -> Self {
        Character {
            name: "Name".to_string(),
            stock: "Stock".to_string(),
            stats: vec![
                Stat::new("Will".into()),
                Stat::new("Perception".into()),
                Stat::new("Conscious".into()),
                Stat::new("Power".into()),
                Stat::new("Speed".into()),
                Stat::new("Forte".into()),
            ],
            skills: Vec::new(),
            quirks: Vec::new(),
            finite_resources: Vec::new(),
            infinite_resources: Vec::new(),
            inspiration: Inspiration::new(),
            argos: String::new(),
            inventory: Vec::new(),
        }
    }
}

/// A struct for Stats.
///
/// `checks` is optional as some stats don't
/// require checks to function.
#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Stat {
    pub name: String,
    pub quality: Quality,
    pub quantity: usize,
    pub checks: Option<usize>,
}

impl Stat {
    #[must_use]
    pub fn new(name: String) -> Stat {
        Stat {
            name,
            quality: Quality::Basic,
            quantity: 1,
            checks: Some(0),
        }
    }
}

impl From<String> for Stat {
    /// Given in the form `{Quality}{Quantity}`.
    /// No `name` or `checks` field are accepted.
    fn from(value: String) -> Self {
        if let Some(first_char) = value.chars().next() {
            let quality = match first_char {
                'A' | 'a' => Quality::Adept,
                'S' | 's' => Quality::Superb,
                _ => Quality::Basic,
            };
            let quantity = value[1..].parse::<usize>().unwrap_or(1);
            Stat {
                name: String::new(),
                quality,
                quantity,
                checks: Some(0),
            }
        } else {
            Stat::new(String::new())
        }
    }
}

impl std::fmt::Display for Stat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.quality {
            Quality::Basic => write!(f, "B{}", self.quantity),
            Quality::Adept => write!(f, "A{}", self.quantity),
            Quality::Superb => write!(f, "S{}", self.quantity),
        }
    }
}

/// A struct for Quality. Determines the
/// lower bound for rolls.
#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Quality {
    Basic = 4,
    Adept = 3,
    Superb = 2,
}

impl std::fmt::Display for Quality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Quality::Basic => write!(f, "Basic"),
            Quality::Adept => write!(f, "Adept"),
            Quality::Superb => write!(f, "Superb"),
        }
    }
}

/// A struct for items.
#[derive(Encode, Decode, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Item {
    pub name: String,
    pub quantity: usize,
    pub description: String,
}

impl Item {
    #[must_use]
    pub fn new(name: String) -> Self {
        Self {
            name,
            quantity: 0,
            description: String::new(),
        }
    }
}

impl Default for Item {
    fn default() -> Self {
        Self::new("New Item!".into())
    }
}
