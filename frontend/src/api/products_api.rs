use crate::api::client::{get_url, handle_error, handle_network_error};
use crate::models::product::{CreateProductDto, Product, ProductDetailResponse};
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

pub async fn fetch_product_by_id(id: i64) -> Result<ProductDetailResponse, String> {
    let client = Client::new();
    let res = client
        .get(&get_url(&format!("/products/{}", id)))
        .send()
        .await
        .map_err(handle_network_error)?;

    if res.status().is_success() {
        res.json::<ProductDetailResponse>().await.map_err(|e| e.to_string())
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

pub async fn upload_product_images(product_id: i64, files: Vec<web_sys::File>, image_url: Option<String>) -> Result<(), String> {
    use gloo_net::http::Request;
    use web_sys::FormData;

    let form = FormData::new().map_err(|_| "Error creando FormData".to_string())?;

    if let Some(url) = image_url {
        if !url.is_empty() {
            form.append_with_str("image_url", &url).map_err(|_| "Error añadiendo URL".to_string())?;
        }
    }

    for file in files {
        form.append_with_blob(&file.name(), &file).map_err(|_| "Error añadiendo archivo".to_string())?;
    }

    let url = get_url(&format!("/products/{}/images", product_id));
    let res = Request::post(&url)
        .body(form)
        .map_err(|e| format!("Error creando request: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Error de red: {}", e))?;

    if res.ok() {
        Ok(())
    } else {
        Err(format!("Error en el servidor: {}", res.status()))
    }
}
