use serde::Deserialize;
use std::fmt;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")] // so JSON serializes to "Gold", "Silver", etc.
pub enum Rating {
    Halal,
    Unconfirmed,
    Unknown,
    Haram,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")] // so JSON serializes to "Gold", "Silver", etc.
pub enum City {
    Delhi,
    Noida,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")] // so JSON serializes to "Gold", "Silver", etc.
pub enum SpotType {
    Restaurant,
    Hotel,
    Meatshop,
    Street,
}

impl fmt::Display for Rating {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let readable = match self {
            Rating::Halal => "Halal Food ✅",
            Rating::Unconfirmed => "Unconfirmed 🟡",
            Rating::Unknown => "Unknown ❓",
            Rating::Haram => "Haram ❌",
        };
        write!(f, "{}", readable)
    }
}
impl fmt::Display for SpotType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let readable = match self {
            SpotType::Restaurant => "Restaurant 🍽️",
            SpotType::Hotel => "Hotel 🏨",
            SpotType::Meatshop => "Meat shop 🔪",
            SpotType::Street => "Street Food 🧆",
        };
        write!(f, "{}", readable)
    }
}
