use crate::models::enums::{City, Rating, SpotType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct NewPlace {
    pub name: String,
    pub image_url: String,
    pub halal_label: Rating,
    pub locality_id: i32,
    pub address: String,
    pub recommended: bool,
    pub place_description: String,
    pub label_description: String,
    pub map_url: String,
    pub mobile_number: String,
    pub place_type: SpotType,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct NewLocality {
    pub name: String,
    pub country_code: String,
    pub city: City,
    pub latitude: f64,
    pub longitude: f64,
    pub locality_verifier: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Place {
    pub id: i32,
    pub name: String,
    pub image_url: String,
    pub halal_label: Rating,
    pub locality_id: i32,
    pub address: String,
    pub recommended: bool,
    pub place_description: String,
    pub label_description: String,
    pub map_url: String,
    pub mobile_number: String,
    pub place_type: SpotType,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Locality {
    pub id: i32,
    pub name: String,
    pub country_code: String,
    pub city: City,
    pub latitude: f64,
    pub longitude: f64,
    pub locality_verifier: String,
}
