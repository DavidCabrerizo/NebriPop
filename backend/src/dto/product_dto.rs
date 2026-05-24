use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateProductDto {
    pub user_id: Option<i64>,
    pub category_id: i64,
    pub title: String,
    pub description: String,
    pub price: f64,
    pub condition: String,
    pub location: String,
    pub main_image_url: Option<String>,
}
