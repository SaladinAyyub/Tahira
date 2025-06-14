use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::models::place::{Locality, NewLocality, NewPlace, Place};

pub async fn add_place(
    State(pool): State<PgPool>,
    Json(place): Json<NewPlace>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let _resp = sqlx::query("INSERT INTO places (name, image_url, halal_label, locality_id, address, recommended, place_description, label_description, map_url, mobile_number, place_type) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)")
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
        .bind(&place.place_type)
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
    Json(locality): Json<NewLocality>,
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

pub async fn get_places(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Place>>, (StatusCode, String)> {
    let result = sqlx::query_as("SELECT * from places")
        .fetch_all(&pool)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error is {}", err),
            )
        })?;
    Ok(Json(result))
}

pub async fn get_localities(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Locality>>, (StatusCode, String)> {
    let result = sqlx::query_as("SELECT * from localities")
        .fetch_all(&pool)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error is {}", err),
            )
        })?;
    Ok(Json(result))
}

pub async fn get_place_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Place>, (StatusCode, String)> {
    let result = sqlx::query_as(
        "SELECT id, name, image_url, halal_label,locality_id, address, recommended, place_description, label_description, map_url, mobile_number, place_type FROM places WHERE id = $1",
    ).bind(id).fetch_one(&pool).await.map_err(|err| match err {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, format!("Error is {}", err)),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error is {}", err))
    })?;
    Ok(Json(result))
}

pub async fn get_locality_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Locality>, (StatusCode, String)> {
    let result = sqlx::query_as(
        "SELECT id, name, country_code, city, latitude, longitude, locality_verifier FROM localities WHERE id = $1",
    ).bind(id).fetch_one(&pool).await.map_err(|err| match err {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, format!("Error is {}", err)),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error is {}", err))
    })?;
    Ok(Json(result))
}

pub async fn delete_place_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let _result = sqlx::query("DELETE FROM places WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, format!("Error is {}", err)),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error is {}", err),
            ),
        })?;
    Ok(Json(json!({"msg": "Place deleted successfully"})))
}

pub async fn delete_locality_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let _result = sqlx::query("DELETE FROM localities WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, format!("Error is {}", err)),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error is {}", err),
            ),
        })?;
    Ok(Json(json!({"msg": "Locality deleted successfully"})))
}

pub async fn update_place_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(place): Json<Place>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let _result = sqlx::query("UPDATE places set name=$1, image_url=$2, halal_label=$3, locality_id=$4, address=$5, recommended=$6, place_description=$7, label_description=$8, map_url=$9, mobile_number=$10, place_type=$11 WHERE id=$12")
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
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, format!("Error is {}", err)),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error is {}", err),
            ),
        })?;
    Ok(Json(json!({"msg": "Place updated successfully"})))
}

pub async fn update_locality_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(locality): Json<Locality>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let _result = sqlx::query("UPDATE localities set name=$1, country_code=$2, city=$3, latitude=$4, longitude=$5, locality_verifier=$6 WHERE id = $7")
        .bind(&locality.name)
        .bind(&locality.country_code)
        .bind(&locality.city)
        .bind(locality.latitude)
        .bind(locality.longitude)
        .bind(&locality.locality_verifier)
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, format!("Error is {}", err)),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error is {}", err),
            ),
        })?;
    Ok(Json(json!({"msg": "Locality updated successfully"})))
}
