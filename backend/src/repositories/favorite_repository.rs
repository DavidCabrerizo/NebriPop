use sqlx::SqlitePool;
use crate::models::Product;
use crate::errors::AppError;

pub struct FavoriteRepository;

impl FavoriteRepository {
    pub async fn add(pool: &SqlitePool, user_id: i64, product_id: i64) -> Result<(), AppError> {
        let result = sqlx::query("INSERT INTO favorites (user_id, product_id) VALUES (?, ?)")
            .bind(user_id)
            .bind(product_id)
            .execute(pool)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(sqlx::Error::Database(e)) if e.is_unique_violation() => {
                Err(AppError::ValidationError("El producto ya está en favoritos".to_string()))
            }
            Err(e) => Err(AppError::DatabaseError(e)),
        }
    }

    pub async fn remove(pool: &SqlitePool, user_id: i64, product_id: i64) -> Result<(), AppError> {
        sqlx::query("DELETE FROM favorites WHERE user_id = ? AND product_id = ?")
            .bind(user_id)
            .bind(product_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn get_user_favorites(pool: &SqlitePool, user_id: i64) -> Result<Vec<Product>, AppError> {
        let products = sqlx::query_as::<_, Product>(
            r#"
            SELECT p.*
            FROM products p
            INNER JOIN favorites f ON p.id = f.product_id
            WHERE f.user_id = ?
            ORDER BY f.created_at DESC
            "#
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;
        
        Ok(products)
    }

    pub async fn is_favorite(pool: &SqlitePool, user_id: i64, product_id: i64) -> Result<bool, AppError> {
        let (exists,): (bool,) = sqlx::query_as(
            "SELECT EXISTS(SELECT 1 FROM favorites WHERE user_id = ? AND product_id = ?)"
        )
        .bind(user_id)
        .bind(product_id)
        .fetch_one(pool)
        .await?;
        
        Ok(exists)
    }
}
