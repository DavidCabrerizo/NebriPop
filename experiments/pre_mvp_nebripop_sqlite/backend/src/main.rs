use axum::{
    extract::State,
    http::{Method, StatusCode},
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Product {
    id: i64,
    title: String,
    description: String,
    price: f64,
    category: String,
    condition: String,
    location: String,
    contact: String,
    created_at: String,
}

#[derive(Debug, Deserialize)]
struct NewProduct {
    title: String,
    description: String,
    price: f64,
    category: String,
    condition: String,
    location: String,
    contact: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    message: String,
}

struct AppState {
    db: Arc<Mutex<Connection>>,
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_headers(Any);

    // Conectar a SQLite de forma persistente. La ruta es relativa al root de la ejecución (backend/)
    let db_path = "../database/nebripop_pre_mvp.db";
    let conn = Connection::open(db_path).expect("No se pudo abrir la base de datos");

    // Inicializar esquema
    let schema = std::fs::read_to_string("../database/schema.sql").unwrap_or_else(|_| "
        CREATE TABLE IF NOT EXISTS products (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            price REAL NOT NULL,
            category TEXT NOT NULL,
            condition TEXT NOT NULL,
            location TEXT NOT NULL,
            contact TEXT NOT NULL,
            created_at TEXT NOT NULL
        );".to_string());
    
    conn.execute_batch(&schema).expect("Fallo al inicializar el esquema SQL");

    // Productos por defecto si está vacío
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM products", [], |row| row.get(0)).unwrap_or(0);
    if count == 0 {
        let now = Utc::now().to_rfc3339();
        conn.execute("INSERT INTO products (title, description, price, category, condition, location, contact, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)", 
            params!["Libro de Programación en Rust", "Libro usado para aprender Rust desde cero", 20.0, "Libros", "Usado", "Campus Nebrija", "usuario_prueba", now]).unwrap();
        conn.execute("INSERT INTO products (title, description, price, category, condition, location, contact, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)", 
            params!["Calculadora científica", "Casio fx-991", 15.0, "Electrónica", "Bueno", "Campus Princesa", "estudiante_2", now]).unwrap();
    }

    let shared_state = Arc::new(AppState {
        db: Arc::new(Mutex::new(conn)),
    });

    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/products", get(get_products).post(create_product))
        .layer(cors)
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Pre-MVP Backend escuchando en http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn health_handler() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        message: "Pre-MVP NebriPop backend funcionando".to_string(),
    })
}

async fn get_products(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Product>>, (StatusCode, Json<ErrorResponse>)> {
    let db = state.db.clone();
    
    let products = tokio::task::spawn_blocking(move || {
        let conn = db.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, title, description, price, category, condition, location, contact, created_at FROM products ORDER BY id DESC").unwrap();
        let prod_iter = stmt.query_map([], |row| {
            Ok(Product {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                price: row.get(3)?,
                category: row.get(4)?,
                condition: row.get(5)?,
                location: row.get(6)?,
                contact: row.get(7)?,
                created_at: row.get(8)?,
            })
        }).unwrap();
        
        let mut result = Vec::new();
        for p in prod_iter {
            if let Ok(product) = p {
                result.push(product);
            }
        }
        result
    }).await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: "Error en concurrencia DB".to_string() })))?;

    Ok(Json(products))
}

async fn create_product(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<NewProduct>,
) -> Result<Json<Product>, (StatusCode, Json<ErrorResponse>)> {
    
    if payload.title.trim().is_empty() { return error_response("title no puede estar vacío"); }
    if payload.description.trim().is_empty() { return error_response("description no puede estar vacía"); }
    if payload.price <= 0.0 { return error_response("price debe ser mayor que 0"); }
    if payload.category.trim().is_empty() { return error_response("category no puede estar vacía"); }
    if payload.condition.trim().is_empty() { return error_response("condition no puede estar vacío"); }
    if payload.location.trim().is_empty() { return error_response("location no puede estar vacía"); }
    if payload.contact.trim().is_empty() { return error_response("contact no puede estar vacío"); }

    let db = state.db.clone();
    
    let product = tokio::task::spawn_blocking(move || {
        let conn = db.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        
        conn.execute(
            "INSERT INTO products (title, description, price, category, condition, location, contact, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![payload.title, payload.description, payload.price, payload.category, payload.condition, payload.location, payload.contact, now],
        ).unwrap();
        
        let id = conn.last_insert_rowid();
        
        Product {
            id,
            title: payload.title,
            description: payload.description,
            price: payload.price,
            category: payload.category,
            condition: payload.condition,
            location: payload.location,
            contact: payload.contact,
            created_at: now,
        }
    }).await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: "Error ejecutando escritura en base de datos".to_string() })))?;

    Ok(Json(product))
}

fn error_response(msg: &str) -> Result<Json<Product>, (StatusCode, Json<ErrorResponse>)> {
    Err((StatusCode::BAD_REQUEST, Json(ErrorResponse { error: msg.to_string() })))
}
