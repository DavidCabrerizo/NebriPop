use reqwest::Client;
use serde_json::json;
use crate::models::user::User;
use crate::models::product::Product;
use crate::models::ApiError;

const API_BASE_URL: &str = "http://127.0.0.1:3000";

pub async fn get_user_profile(id: i64) -> Result<User, String> {
    let client = Client::new();
    let res = client.get(format!("{}/users/{}", API_BASE_URL, id))
        .send()
        .await
        .map_err(|e| format!("Error de conexión: {}", e))?;

    if res.status().is_success() {
        res.json::<User>().await.map_err(|e| format!("Error al parsear respuesta: {}", e))
    } else {
        let error_body = res.json::<ApiError>().await.unwrap_or(ApiError { error: "Error desconocido".into() });
        Err(error_body.error)
    }
}

pub async fn get_user_products(id: i64) -> Result<Vec<Product>, String> {
    let client = Client::new();
    let res = client.get(format!("{}/users/{}/products", API_BASE_URL, id))
        .send()
        .await
        .map_err(|e| format!("Error de conexión: {}", e))?;

    if res.status().is_success() {
        res.json::<Vec<Product>>().await.map_err(|e| format!("Error al parsear respuesta: {}", e))
    } else {
        let error_body = res.json::<ApiError>().await.unwrap_or(ApiError { error: "Error desconocido".into() });
        Err(error_body.error)
    }
}

pub async fn update_user_profile(id: i64, phone: Option<&str>, location: Option<&str>) -> Result<User, String> {
    let client = Client::new();
    let res = client.put(format!("{}/users/{}", API_BASE_URL, id))
        .json(&json!({
            "phone": phone,
            "location": location,
        }))
        .send()
        .await
        .map_err(|e| format!("Error de conexión: {}", e))?;

    if res.status().is_success() {
        res.json::<User>().await.map_err(|e| format!("Error al parsear respuesta: {}", e))
    } else {
        let error_body = res.json::<ApiError>().await.unwrap_or(ApiError { error: "Error desconocido".into() });
        Err(error_body.error)
    }
}
