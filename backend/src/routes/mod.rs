use axum::{
    routing::{get, post},
    Router,
};
use sqlx::SqlitePool;
use tower_http::services::ServeDir;

use crate::handlers::{
    categories_handler, health_handler, products_handler,
};

pub fn app_router(pool: SqlitePool) -> Router {
    Router::new()
        .nest_service("/uploads", ServeDir::new("uploads"))
        .route("/health", get(health_handler::health_check))
        .route("/categories", get(categories_handler::get_categories))
        .route("/products", get(products_handler::get_products))
        .route("/products", post(products_handler::create_product))
        .route("/products/:id", get(products_handler::get_product_by_id))
        .route("/products/:id/images", post(products_handler::upload_product_images))
        .with_state(pool)
}
