use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Product {
    pub id: i64,
    pub user_id: Option<i64>,
    pub category_id: i64,
    pub title: String,
    pub description: String,
    pub price: f64,
    pub condition: String,
    pub location: String,
    pub status: String,
    pub main_image_url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
