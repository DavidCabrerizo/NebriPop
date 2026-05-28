use axum::{
    routing::{get, post},
    Router,
};
use sqlx::SqlitePool;

use crate::handlers::favorites_handler;

pub fn favorites_routes() -> Router<SqlitePool> {
    Router::new()
        .route("/favorites", post(favorites_handler::add_favorite))
        .route("/favorites/check", get(favorites_handler::check_favorite))
        .route("/favorites/:product_id", axum::routing::delete(favorites_handler::remove_favorite))
        .route("/users/:id/favorites", get(favorites_handler::get_user_favorites))
}
