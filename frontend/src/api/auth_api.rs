use reqwest::Client;
use serde_json::json;
use crate::models::auth::AuthResponse;
use crate::models::ApiError;

const API_BASE_URL: &str = "http://127.0.0.1:3000";

pub async fn register(name: &str, email: &str, password: &str, phone: Option<&str>, location: Option<&str>) -> Result<AuthResponse, String> {
    let client = Client::new();
    let res = client.post(format!("{}/auth/register", API_BASE_URL))
        .json(&json!({
            "name": name,
            "email": email,
            "password": password,
            "phone": phone,
            "location": location,
        }))
        .send()
        .await
        .map_err(|e| format!("Error de conexión: {}", e))?;

    if res.status().is_success() {
        res.json::<AuthResponse>().await.map_err(|e| format!("Error al parsear respuesta: {}", e))
    } else {
        let error_body = res.json::<ApiError>().await.unwrap_or(ApiError { error: "Error desconocido".into() });
        Err(error_body.error)
    }
}

pub async fn login(email: &str, password: &str) -> Result<AuthResponse, String> {
    let client = Client::new();
    let res = client.post(format!("{}/auth/login", API_BASE_URL))
        .json(&json!({
            "email": email,
            "password": password,
        }))
        .send()
        .await
        .map_err(|e| format!("Error de conexión: {}", e))?;

    if res.status().is_success() {
        res.json::<AuthResponse>().await.map_err(|e| format!("Error al parsear respuesta: {}", e))
    } else {
        let error_body = res.json::<ApiError>().await.unwrap_or(ApiError { error: "Error desconocido".into() });
        Err(error_body.error)
    }
}
