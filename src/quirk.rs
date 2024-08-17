use serde::{Deserialize, Serialize};

use bitcode::{Decode, Encode};

/// A struct for Quirks. Boons
/// and flaws are optional as some
/// Quirks are purely cosmetic/neutral.
#[derive(Encode, Decode, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Quirk {
    pub name: String,
    pub category: QuirkCategory,
    pub description: String,
    pub boons: Vec<String>,
    pub flaws: Vec<String>,
}

impl Quirk {
    #[must_use]
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: String::new(),
            category: QuirkCategory::Ethos,
            boons: vec![],
            flaws: vec![],
        }
    }
}

impl Default for Quirk {
    fn default() -> Self {
        Self::new("New Quirk!".into())
    }
}

/// The Quirk category.
#[derive(Encode, Decode, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum QuirkCategory {
    Ethos,
    Pathos,
    Logos,
    Uncategorized,
}

#[derive(Encode, Decode, Serialize, Deserialize, Default, Clone, PartialEq, Debug)]
pub struct Inspiration {
    pub ethos: usize,
    pub pathos: usize,
    pub logos: usize,
}

impl Inspiration {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
