pub mod category;
pub mod product;
pub mod user;
pub mod favorite;
pub mod message;
pub mod auth;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApiError {
    pub error: String,
}
