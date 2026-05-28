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
    pub status: String,
    pub main_image_url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateProductDto {
    pub user_id: i64,
    pub category_id: i64,
    pub title: String,
    pub description: String,
    pub price: f64,
    pub condition: String,
    pub location: String,
    pub status: Option<String>,
    pub main_image_url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ProductDetailResponse {
    pub product: Product,
    pub author_name: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProductFilters {
    pub search: String,
    pub category_id: String,
    pub min_price: String,
    pub max_price: String,
    pub condition: String,
    pub location: String,
    pub status: String,
    pub sort: String,
}

impl ProductFilters {
    pub fn to_query_string(&self) -> String {
        let mut params = Vec::new();

        if !self.search.trim().is_empty() {
            params.push(format!("search={}", urlencoding::encode(&self.search)));
        }
        if !self.category_id.trim().is_empty() {
            params.push(format!("category_id={}", self.category_id));
        }
        if !self.min_price.trim().is_empty() {
            params.push(format!("min_price={}", self.min_price));
        }
        if !self.max_price.trim().is_empty() {
            params.push(format!("max_price={}", self.max_price));
        }
        if !self.condition.trim().is_empty() && self.condition != "all" {
            params.push(format!("condition={}", urlencoding::encode(&self.condition)));
        }
        if !self.location.trim().is_empty() {
            params.push(format!("location={}", urlencoding::encode(&self.location)));
        }
        if !self.status.trim().is_empty() && self.status != "all" {
            params.push(format!("status={}", urlencoding::encode(&self.status)));
        }
        if !self.sort.trim().is_empty() {
            params.push(format!("sort={}", urlencoding::encode(&self.sort)));
        }

        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }
}
