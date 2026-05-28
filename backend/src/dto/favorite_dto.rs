use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AddFavoriteDto {
    pub user_id: i64,
    pub product_id: i64,
}

#[derive(Serialize)]
pub struct FavoriteResponse {
    pub message: String,
    pub user_id: i64,
    pub product_id: i64,
}

#[derive(Serialize)]
pub struct FavoriteCheckResponse {
    pub is_favorite: bool,
}
