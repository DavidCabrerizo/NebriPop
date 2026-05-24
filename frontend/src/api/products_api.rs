use crate::api::client::{get_url, handle_error, handle_network_error};
use crate::models::product::{CreateProductDto, Product};
use reqwest::Client;

pub async fn fetch_products() -> Result<Vec<Product>, String> {
    let client = Client::new();
    let res = client
        .get(&get_url("/products"))
        .send()
        .await
        .map_err(handle_network_error)?;

    if res.status().is_success() {
        res.json::<Vec<Product>>().await.map_err(|e| e.to_string())
    } else {
        Err(handle_error(res).await)
    }
}

pub async fn fetch_product_by_id(id: i64) -> Result<Product, String> {
    let client = Client::new();
    let res = client
        .get(&get_url(&format!("/products/{}", id)))
        .send()
        .await
        .map_err(handle_network_error)?;

    if res.status().is_success() {
        res.json::<Product>().await.map_err(|e| e.to_string())
    } else {
        Err(handle_error(res).await)
    }
}

pub async fn create_product(dto: CreateProductDto) -> Result<Product, String> {
    let client = Client::new();
    let res = client
        .post(&get_url("/products"))
        .json(&dto)
        .send()
        .await
        .map_err(handle_network_error)?;

    if res.status().is_success() {
        res.json::<Product>().await.map_err(|e| e.to_string())
    } else {
        Err(handle_error(res).await)
    }
}
