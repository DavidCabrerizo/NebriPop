use axum::{
    routing::{get, put},
    Router,
};
use sqlx::SqlitePool;

use crate::handlers::users_handler::{get_user_profile, get_user_products, update_user_profile};

pub fn users_routes() -> Router<SqlitePool> {
    Router::new()
        .route("/:id", get(get_user_profile))
        .route("/:id", put(update_user_profile))
        .route("/:id/products", get(get_user_products))
}
