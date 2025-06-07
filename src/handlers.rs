use axum::{Json, extract::State, http::StatusCode};
use serde_json::{Value, json};
use sqlx::PgPool;

use crate::models::place::{Locality, Place};

pub async fn add_place(
    State(pool): State<PgPool>,
    Json(place): Json<Place>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let _resp = sqlx::query("INSERT INTO places (name, image_url, halal_label, locality, address, recommended, place_description, label_description, map_url, mobile_number) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)")
        .bind(&place.name)
        .bind(&place.image_url)
        .bind(&place.halal_label)
        .bind(place.locality_id)
        .bind(&place.address)
        .bind(place.recommended)
        .bind(&place.place_description)
        .bind(&place.label_description)
        .bind(&place.map_url)
        .bind(&place.mobile_number)
        .execute(&pool)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error is: {}", err),
            )
        })?;
    Ok(Json(json!(place)))
}

pub async fn add_locality(
    State(pool): State<PgPool>,
    Json(locality): Json<Locality>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let _resp =
        sqlx::query("INSERT INTO localities (name, country_code, city, latitude, longitude, locality_verifier) values ($1, $2, $3, $4, $5, $6)")
            .bind(&locality.name)
            .bind(&locality.country_code)
            .bind(&locality.city)
            .bind(locality.latitude)
            .bind(locality.longitude)
            .bind(&locality.locality_verifier)
            .execute(&pool)
            .await
            .map_err(|err| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Error is: {}", err),
                )
            })?;
    Ok(Json(json!(locality)))
}
