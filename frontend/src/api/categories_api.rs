use crate::api::client::{get_url, handle_error, handle_network_error};
use crate::models::category::Category;
use reqwest::Client;

pub async fn fetch_categories() -> Result<Vec<Category>, String> {
    let client = Client::new();
    let res = client
        .get(&get_url("/categories"))
        .send()
        .await
        .map_err(handle_network_error)?;

    if res.status().is_success() {
        res.json::<Vec<Category>>().await.map_err(|e| e.to_string())
    } else {
        Err(handle_error(res).await)
    }
}
