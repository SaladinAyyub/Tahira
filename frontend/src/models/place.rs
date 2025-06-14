use crate::models::enums::{Rating, SpotType};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct PlaceCardProps {
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
