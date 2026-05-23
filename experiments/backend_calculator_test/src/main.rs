use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

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
    // Inicialización del estado compartido en memoria
    let shared_state = Arc::new(AppState {
        operations: Mutex::new(Vec::new()),
        next_id: Mutex::new(1),
    });

    // Construcción del enrutador (Router)
    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/operations", get(get_operations).post(create_operation))
        .with_state(shared_state);

    // Servidor escuchando en el puerto 3000
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
    
    // 1. Validar operador
    let valid_ops = ["+", "-", "*", "/"];
    if !valid_ops.contains(&payload.operator.as_str()) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Operador no válido. Use +, -, * o /".to_string(),
            }),
        ));
    }

    // 2. Validar división entre cero
    if payload.operator == "/" && payload.right_value == 0.0 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "División por cero no permitida".to_string(),
            }),
        ));
    }

    // 3. Validar consistencia matemática del resultado
    let expected_result = match payload.operator.as_str() {
        "+" => payload.left_value + payload.right_value,
        "-" => payload.left_value - payload.right_value,
        "*" => payload.left_value * payload.right_value,
        "/" => payload.left_value / payload.right_value,
        _ => 0.0,
    };

    // Usamos un margen de error pequeño para las operaciones con punto flotante
    if (payload.result - expected_result).abs() > 0.0001 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "El resultado indicado no es correcto para esta operación".to_string(),
            }),
        ));
    }

    // Almacenamos la operación exitosa
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
