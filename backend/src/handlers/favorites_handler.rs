use axum::{
    extract::{Path, Query, State},
    Json,
};
use sqlx::SqlitePool;
use std::collections::HashMap;

use crate::{
    dto::favorite_dto::{AddFavoriteDto, FavoriteCheckResponse, FavoriteResponse},
    errors::AppError,
    models::Product,
    repositories::{favorite_repository::FavoriteRepository, product_repository::ProductRepository, user_repository},
};

pub async fn get_user_favorites(
    State(pool): State<SqlitePool>,
    Path(user_id): Path<i64>,
) -> Result<Json<Vec<Product>>, AppError> {
    let _user = user_repository::find_by_id(&pool, user_id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Usuario con ID {} no encontrado", user_id)))?;

    let favorites = FavoriteRepository::get_user_favorites(&pool, user_id).await?;
    Ok(Json(favorites))
}

pub async fn add_favorite(
    State(pool): State<SqlitePool>,
    Json(payload): Json<AddFavoriteDto>,
) -> Result<Json<FavoriteResponse>, AppError> {
    let _user = user_repository::find_by_id(&pool, payload.user_id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Usuario con ID {} no encontrado", payload.user_id)))?;

    let _product = ProductRepository::find_by_id(&pool, payload.product_id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Producto con ID {} no encontrado", payload.product_id)))?;

    FavoriteRepository::add(&pool, payload.user_id, payload.product_id).await?;

    Ok(Json(FavoriteResponse {
        message: "Producto añadido a favoritos".to_string(),
        user_id: payload.user_id,
        product_id: payload.product_id,
    }))
}

pub async fn remove_favorite(
    State(pool): State<SqlitePool>,
    Path(product_id): Path<i64>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<FavoriteResponse>, AppError> {
    let user_id = params
        .get("user_id")
        .and_then(|id| id.parse::<i64>().ok())
        .ok_or_else(|| AppError::ValidationError("Falta user_id".to_string()))?;

    // Check if it exists
    let is_fav = FavoriteRepository::is_favorite(&pool, user_id, product_id).await?;
    if !is_fav {
        return Err(AppError::NotFound("El producto no está en favoritos".to_string()));
    }

    FavoriteRepository::remove(&pool, user_id, product_id).await?;

    Ok(Json(FavoriteResponse {
        message: "Producto eliminado de favoritos".to_string(),
        user_id,
        product_id,
    }))
}

pub async fn check_favorite(
    State(pool): State<SqlitePool>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<FavoriteCheckResponse>, AppError> {
    let user_id = params
        .get("user_id")
        .and_then(|id| id.parse::<i64>().ok())
        .ok_or_else(|| AppError::ValidationError("Falta user_id".to_string()))?;

    let product_id = params
        .get("product_id")
        .and_then(|id| id.parse::<i64>().ok())
        .ok_or_else(|| AppError::ValidationError("Falta product_id".to_string()))?;

    let is_favorite = FavoriteRepository::is_favorite(&pool, user_id, product_id).await?;

    Ok(Json(FavoriteCheckResponse { is_favorite }))
}
