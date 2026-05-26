use axum::{
    extract::{Path, State},
    response::Json,
};
use sqlx::SqlitePool;

use crate::errors::AppError;
use crate::models::{User, Product};
use crate::dto::user_dto::UpdateProfileRequest;
use crate::repositories::user_repository;
use crate::repositories::product_repository;

pub async fn get_user_profile(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<User>, AppError> {
    let user = user_repository::find_by_id(&pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Usuario {} no encontrado", id)))?;

    Ok(Json(user))
}

pub async fn get_user_products(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<Vec<Product>>, AppError> {
    // Reutilizamos la función del repositorio de productos o hacemos una nueva
    // Depende de si existe find_by_user_id en ProductRepository.
    // Vamos a crearla o usar una query en product_repository
    let products = product_repository::ProductRepository::find_by_user_id(&pool, id).await?;
    Ok(Json(products))
}

pub async fn update_user_profile(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<Json<User>, AppError> {
    let user = user_repository::update_profile(
        &pool,
        id,
        payload.phone.as_deref(),
        payload.location.as_deref(),
    ).await?;

    Ok(Json(user))
}
