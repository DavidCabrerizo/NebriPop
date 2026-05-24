use serde::Deserialize;

pub const API_BASE_URL: &str = "http://127.0.0.1:3000";

pub fn get_url(path: &str) -> String {
    format!("{}{}", API_BASE_URL, path)
}

#[derive(Deserialize)]
struct ApiError {
    error: String,
}

pub async fn handle_error(res: reqwest::Response) -> String {
    let status = res.status();
    let text = res.text().await.unwrap_or_default();
    
    if let Ok(api_err) = serde_json::from_str::<ApiError>(&text) {
        api_err.error
    } else if !text.is_empty() {
        format!("Error {}: {}", status, text)
    } else {
        format!("Error del servidor: {}", status)
    }
}

pub fn handle_network_error(err: reqwest::Error) -> String {
    format!("Error de conexión al servidor: {}", err)
}
