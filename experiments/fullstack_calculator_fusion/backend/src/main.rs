use axum::{
    extract::State,
    http::{Method, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};

// --- Models ---
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Operation {
    id: u64,
    left_value: f64,
    operator: String,
    right_value: f64,
    result: f64,
}

#[derive(Debug, Deserialize)]
struct NewOperation {
    left_value: f64,
    operator: String,
    right_value: f64,
    result: f64,
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

// --- App State ---
struct AppState {
    operations: Mutex<Vec<Operation>>,
    next_id: Mutex<u64>,
}

#[tokio::main]
async fn main() {
    // Configuración del CORS para permitir llamadas desde el frontend
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_headers(Any);

    let shared_state = Arc::new(AppState {
        operations: Mutex::new(Vec::new()),
        next_id: Mutex::new(1),
    });

    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/operations", get(get_operations).post(create_operation))
        .layer(cors)
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Servidor backend escuchando en http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

// --- Handlers ---
async fn health_handler() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        message: "Backend de prueba funcionando".to_string(),
    })
}

async fn get_operations(
    State(state): State<Arc<AppState>>,
) -> Json<Vec<Operation>> {
    let ops = state.operations.lock().unwrap().clone();
    Json(ops)
}

async fn create_operation(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<NewOperation>,
) -> Result<Json<Operation>, (StatusCode, Json<ErrorResponse>)> {
    
    let valid_ops = ["+", "-", "*", "/"];
    if !valid_ops.contains(&payload.operator.as_str()) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Operador no válido. Use +, -, * o /".to_string(),
            }),
        ));
    }

    if payload.operator == "/" && payload.right_value == 0.0 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "División por cero no permitida".to_string(),
            }),
        ));
    }

    let expected_result = match payload.operator.as_str() {
        "+" => payload.left_value + payload.right_value,
        "-" => payload.left_value - payload.right_value,
        "*" => payload.left_value * payload.right_value,
        "/" => payload.left_value / payload.right_value,
        _ => 0.0,
    };

    if (payload.result - expected_result).abs() > 0.0001 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "El resultado indicado no es correcto para esta operación".to_string(),
            }),
        ));
    }

    let mut next_id = state.next_id.lock().unwrap();
    let id = *next_id;
    *next_id += 1;

    let op = Operation {
        id,
        left_value: payload.left_value,
        operator: payload.operator,
        right_value: payload.right_value,
        result: payload.result,
    };

    state.operations.lock().unwrap().push(op.clone());

    Ok(Json(op))
}
