use axum::{extract::State, response::Json};
use sqlx::SqlitePool;
use crate::errors::AppError;
use crate::models::Category;
use crate::repositories::category_repository::CategoryRepository;

pub async fn get_categories(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Category>>, AppError> {
    let categories = CategoryRepository::find_all(&pool).await?;
    Ok(Json(categories))
}
