use axum::{
    routing::post,
    Router,
};
use sqlx::SqlitePool;

use crate::handlers::auth_handler::{login, register};

pub fn auth_routes() -> Router<SqlitePool> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}
