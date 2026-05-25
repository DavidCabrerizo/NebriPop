use axum::{
    routing::get,
    Router,
};
use sqlx::SqlitePool;

use crate::handlers::users_handler::{get_user_profile, get_user_products};

pub fn users_routes() -> Router<SqlitePool> {
    Router::new()
        .route("/:id", get(get_user_profile))
        .route("/:id/products", get(get_user_products))
}
