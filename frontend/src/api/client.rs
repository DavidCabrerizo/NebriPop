pub const API_BASE_URL: &str = "http://127.0.0.1:3000";

pub fn get_url(path: &str) -> String {
    format!("{}{}", API_BASE_URL, path)
}
