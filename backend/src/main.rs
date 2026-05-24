mod config;
mod db;
mod dto;
mod errors;
mod handlers;
mod models;
mod repositories;
mod routes;

use axum::http::Method;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Inicializar el logger
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Cargar configuración
    let cfg = config::Config::init();

    // Inicializar base de datos (concreta para MVP y sqlite local)
    let pool = db::init_db(&cfg.database_url).await?;

    // Configurar CORS
    let cors = CorsLayer::new()
        .allow_origin(Any) // En producción limitar a http://127.0.0.1:8081
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    // Crear la aplicación (router)
    let app = routes::app_router(pool).layer(cors);

    // Bind y arrancar servidor
    let addr = SocketAddr::from(([127, 0, 0, 1], cfg.port));
    tracing::debug!("Escuchando en {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
