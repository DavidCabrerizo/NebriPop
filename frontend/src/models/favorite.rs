use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
pub struct AddFavoriteDto {
    pub user_id: i64,
    pub product_id: i64,
}

#[derive(Deserialize, Clone)]
pub struct FavoriteCheckResponse {
    pub is_favorite: bool,
}
