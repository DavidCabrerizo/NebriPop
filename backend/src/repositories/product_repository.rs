use sqlx::SqlitePool;
use crate::models::Product;
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
        let product = sqlx::query_as::<_, Product>(
            r#"SELECT 
                id, user_id, category_id, title, description, price, condition, 
                location, status, main_image_url, created_at, updated_at 
               FROM products 
               WHERE id = ?"#
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(product)
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
