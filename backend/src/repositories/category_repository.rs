use sqlx::SqlitePool;
use crate::models::Category;
use crate::errors::AppError;

pub struct CategoryRepository;

impl CategoryRepository {
    pub async fn find_all(pool: &SqlitePool) -> Result<Vec<Category>, AppError> {
        let categories = sqlx::query_as::<_, Category>(
            r#"SELECT id, name, created_at FROM categories ORDER BY name"#
        )
        .fetch_all(pool)
        .await?;

        Ok(categories)
    }

    pub async fn find_by_id(pool: &SqlitePool, id: i64) -> Result<Option<Category>, AppError> {
        let category = sqlx::query_as::<_, Category>(
            r#"SELECT id, name, created_at FROM categories WHERE id = ?"#
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(category)
    }
}
