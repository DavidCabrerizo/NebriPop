use axum::{
    extract::{Path, State, Multipart},
    response::Json,
};
use sqlx::SqlitePool;
use std::path::PathBuf;
use tokio::fs;
use crate::dto::product_dto::CreateProductDto;
use crate::errors::AppError;
use crate::models::{Product, ProductImage};
use crate::repositories::{
    category_repository::CategoryRepository,
    product_repository::ProductRepository,
};
use serde::Serialize;

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

    let product = ProductRepository::create(&pool, &payload).await?;

    Ok(Json(product))
}

#[derive(Serialize)]
pub struct UploadImagesResponse {
    pub message: String,
    pub images: Vec<ProductImage>,
}

pub async fn upload_product_images(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    mut multipart: Multipart,
) -> Result<Json<UploadImagesResponse>, AppError> {
    // Comprobar que el producto existe
    let _product = ProductRepository::find_by_id(&pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Producto con ID {} no encontrado", id)))?;

    let upload_dir = format!("uploads/products/{}", id);
    fs::create_dir_all(&upload_dir).await.map_err(|_| AppError::InternalServerError)?;

    let mut saved_images = Vec::new();
    let mut image_count = ProductRepository::find_images_by_product_id(&pool, id).await?.len();

    while let Some(field) = multipart.next_field().await.map_err(|_| AppError::ValidationError("Error procesando multipart".into()))? {
        let name = field.name().unwrap_or("").to_string();
        
        if name == "image_url" {
            if let Ok(text) = field.text().await {
                let text = text.trim();
                if !text.is_empty() {
                    // Validar URL básica
                    if text.starts_with("http://") || text.starts_with("https://") {
                        let saved = ProductRepository::add_image(&pool, id, text).await?;
                        saved_images.push(saved);
                    }
                }
            }
            continue;
        }

        let file_name = field.file_name().unwrap_or("").to_string();
        if file_name.is_empty() {
            continue;
        }

        let ext = std::path::Path::new(&file_name).extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
        if ext != "jpg" && ext != "jpeg" && ext != "png" {
            return Err(AppError::ValidationError(format!("Formato no permitido: {}", ext)));
        }

        let data = field.bytes().await.map_err(|_| AppError::ValidationError("Error leyendo archivo".into()))?;
        if data.len() > 5 * 1024 * 1024 { // 5 MB max
            return Err(AppError::ValidationError("Archivo demasiado grande (max 5MB)".into()));
        }

        image_count += 1;
        let new_file_name = format!("imagen_{}.{}", image_count, ext);
        let file_path = format!("{}/{}", upload_dir, new_file_name);

        fs::write(&file_path, &data).await.map_err(|_| AppError::InternalServerError)?;

        let db_path = format!("uploads/products/{}/{}", id, new_file_name);
        let saved = ProductRepository::add_image(&pool, id, &db_path).await?;
        saved_images.push(saved);
    }

    Ok(Json(UploadImagesResponse {
        message: "Imágenes añadidas correctamente".to_string(),
        images: saved_images,
    }))
}
