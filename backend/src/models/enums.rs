use serde::{Deserialize, Serialize};
use sqlx::Type;
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Type, EnumString, Display)]
#[sqlx(type_name = "rating")] // matches Postgres ENUM type
#[sqlx(rename_all = "PascalCase")] // matches values like "Gold", "Silver", etc.
#[serde(rename_all = "PascalCase")] // so JSON serializes to "Gold", "Silver", etc.
pub enum Rating {
    Halal,
    Unconfirmed,
    Unknown,
    Haram,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Type, EnumString, Display)]
#[sqlx(type_name = "city")] // matches Postgres ENUM type
#[sqlx(rename_all = "PascalCase")] // matches values like "Gold", "Silver", etc.
#[serde(rename_all = "PascalCase")] // so JSON serializes to "Gold", "Silver", etc.
pub enum City {
    Delhi,
    Noida,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Type, EnumString, Display)]
#[sqlx(type_name = "spot_type")] // matches Postgres ENUM type
#[sqlx(rename_all = "PascalCase")] // matches values like "Gold", "Silver", etc.
#[serde(rename_all = "PascalCase")] // so JSON serializes to "Gold", "Silver", etc.
pub enum SpotType {
    Restaurant,
    Hotel,
    Meatshop,
}
