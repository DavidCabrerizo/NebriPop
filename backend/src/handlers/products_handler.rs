use axum::{
    extract::{Path, State},
    response::Json,
};
use sqlx::SqlitePool;
use crate::dto::product_dto::CreateProductDto;
use crate::errors::AppError;
use crate::models::Product;
use crate::repositories::{
    category_repository::CategoryRepository,
    product_repository::ProductRepository,
};

pub async fn get_products(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Product>>, AppError> {
    let products = ProductRepository::find_all(&pool).await?;
    Ok(Json(products))
}

pub async fn get_product_by_id(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<Product>, AppError> {
    let product = ProductRepository::find_by_id(&pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Producto con ID {} no encontrado", id)))?;
    
    Ok(Json(product))
}

pub async fn create_product(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateProductDto>,
) -> Result<Json<Product>, AppError> {
    // Validaciones
    if payload.title.trim().is_empty() {
        return Err(AppError::ValidationError("El título no puede estar vacío".to_string()));
    }
    if payload.description.trim().is_empty() {
        return Err(AppError::ValidationError("La descripción no puede estar vacía".to_string()));
    }
    if payload.price < 0.0 {
        return Err(AppError::ValidationError("El precio debe ser mayor o igual que 0".to_string()));
    }
    
    let valid_conditions = ["new", "like_new", "good", "used", "damaged"];
    if !valid_conditions.contains(&payload.condition.as_str()) {
        return Err(AppError::ValidationError("Condición no válida".to_string()));
    }
    
    if payload.location.trim().is_empty() {
        return Err(AppError::ValidationError("La ubicación no puede estar vacía".to_string()));
    }

    // Comprobar que la categoría existe
    let category = CategoryRepository::find_by_id(&pool, payload.category_id).await?;
    if category.is_none() {
        return Err(AppError::ValidationError(format!("La categoría con ID {} no existe", payload.category_id)));
    }

    // user_id validation skipped due to MVP constraints (no login functional yet), 
    // it can be null or a fixed number for now.

    let product = ProductRepository::create(&pool, &payload).await?;

    Ok(Json(product))
}
