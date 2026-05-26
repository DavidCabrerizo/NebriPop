use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Product {
    pub id: i64,
    pub user_id: i64,
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
    pub images: Option<Vec<ProductImage>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ProductImage {
    pub id: i64,
    pub product_id: i64,
    pub image_url: String,
    pub position: i64,
    pub created_at: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateProductDto {
    pub user_id: i64,
    pub category_id: i64,
    pub title: String,
    pub description: String,
    pub price: f64,
    pub condition: String,
    pub location: String,
    pub main_image_url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ProductDetailResponse {
    pub product: Product,
    pub author_name: String,
}
