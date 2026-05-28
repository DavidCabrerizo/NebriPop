pub mod auth;
pub mod users;
pub mod favorites;
pub mod messages;

use axum::{
    routing::{get, post, put, delete},
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
        .route("/products", get(products_handler::get_products).post(products_handler::create_product))
        .route("/products/:id", get(products_handler::get_product_by_id)
            .put(products_handler::update_product)
            .delete(products_handler::delete_product))
        .route("/products/:id/images", post(products_handler::upload_product_images))
        .route("/products/:product_id/images/:image_id", delete(products_handler::delete_product_image))
        .nest("/auth", auth::auth_routes())
        .nest("/users", users::users_routes())
        .merge(favorites::favorites_routes())
        .with_state(pool)
}
