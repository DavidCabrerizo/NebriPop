use sqlx::SqlitePool;
use crate::models::{Product, ProductImage};
use crate::dto::product_dto::CreateProductDto;
use crate::errors::AppError;

pub struct ProductRepository;

impl ProductRepository {
    pub async fn find_all(pool: &SqlitePool) -> Result<Vec<Product>, AppError> {
        let products = sqlx::query_as::<_, Product>(
            r#"SELECT 
                id, user_id, category_id, title, description, price, condition, 
                location, status, main_image_url, created_at, updated_at 
               FROM products 
               ORDER BY created_at DESC"#
        )
        .fetch_all(pool)
        .await?;

        Ok(products)
    }

    pub async fn find_by_id(pool: &SqlitePool, id: i64) -> Result<Option<Product>, AppError> {
        let mut product = sqlx::query_as::<_, Product>(
            r#"SELECT 
                id, user_id, category_id, title, description, price, condition, 
                location, status, main_image_url, created_at, updated_at 
               FROM products 
               WHERE id = ?"#
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        if let Some(ref mut p) = product {
            p.images = Some(Self::find_images_by_product_id(pool, id).await?);
        }

        Ok(product)
    }

    pub async fn find_images_by_product_id(pool: &SqlitePool, product_id: i64) -> Result<Vec<ProductImage>, AppError> {
        let images = sqlx::query_as::<_, ProductImage>(
            r#"SELECT id, product_id, image_url, position, created_at 
               FROM product_images 
               WHERE product_id = ? 
               ORDER BY position ASC"#
        )
        .bind(product_id)
        .fetch_all(pool)
        .await?;

        Ok(images)
    }

    pub async fn add_image(pool: &SqlitePool, product_id: i64, image_url: &str) -> Result<ProductImage, AppError> {
        let images = Self::find_images_by_product_id(pool, product_id).await?;
        let position = images.len() as i64;

        let result = sqlx::query(
            r#"INSERT INTO product_images (product_id, image_url, position) VALUES (?, ?, ?)"#
        )
        .bind(product_id)
        .bind(image_url)
        .bind(position)
        .execute(pool)
        .await?
        .last_insert_rowid();

        // Si es la primera imagen, actualizamos el main_image_url del producto
        if position == 0 {
            sqlx::query("UPDATE products SET main_image_url = ? WHERE id = ?")
                .bind(image_url)
                .bind(product_id)
                .execute(pool)
                .await?;
        }

        let image = sqlx::query_as::<_, ProductImage>(
            r#"SELECT id, product_id, image_url, position, created_at FROM product_images WHERE id = ?"#
        )
        .bind(result)
        .fetch_one(pool)
        .await?;

        Ok(image)
    }

    pub async fn create(pool: &SqlitePool, dto: &CreateProductDto) -> Result<Product, AppError> {
        let status = "available"; // Default status
        
        let result = sqlx::query(
            r#"INSERT INTO products (
                user_id, category_id, title, description, price, condition, location, status, main_image_url
               ) 
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(dto.user_id)
        .bind(dto.category_id)
        .bind(&dto.title)
        .bind(&dto.description)
        .bind(dto.price)
        .bind(&dto.condition)
        .bind(&dto.location)
        .bind(status)
        .bind(&dto.main_image_url)
        .execute(pool)
        .await?
        .last_insert_rowid();

        // Recuperamos el producto insertado para devolver los datos completos
        let product = Self::find_by_id(pool, result)
            .await?
            .ok_or_else(|| AppError::InternalServerError)?; // No debería pasar si acabamos de insertar

        Ok(product)
    }
}
