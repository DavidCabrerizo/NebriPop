use serde::{Deserialize, Serialize};
use crate::models::Product;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateProductDto {
    pub user_id: Option<i64>,
    pub category_id: i64,
    pub title: String,
    pub description: String,
    pub price: f64,
    pub condition: String,
    pub location: String,
    pub status: Option<String>,
    pub main_image_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProductDetailResponse {
    pub product: Product,
    pub author_name: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct ProductFiltersDto {
    pub search: Option<String>,
    pub category_id: Option<i64>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
    pub condition: Option<String>,
    pub location: Option<String>,
    pub status: Option<String>,
    pub sort: Option<String>,
}
