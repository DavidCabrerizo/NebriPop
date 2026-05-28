use reqwest::Client;
use crate::{api::client::{get_url, handle_error, handle_network_error}, models::{product::Product, favorite::{AddFavoriteDto, FavoriteCheckResponse}}};

pub async fn get_user_favorites(user_id: i64) -> Result<Vec<Product>, String> {
    let client = Client::new();
    let res = client
        .get(&get_url(&format!("/users/{}/favorites", user_id)))
        .send()
        .await
        .map_err(handle_network_error)?;

    if res.status().is_success() {
        res.json::<Vec<Product>>().await.map_err(|e| format!("Error al leer favoritos: {}", e))
    } else {
        Err(handle_error(res).await)
    }
}

pub async fn add_favorite(user_id: i64, product_id: i64) -> Result<(), String> {
    let client = Client::new();
    let res = client
        .post(&get_url("/favorites"))
        .json(&AddFavoriteDto { user_id, product_id })
        .send()
        .await
        .map_err(handle_network_error)?;

    if res.status().is_success() {
        Ok(())
    } else {
        Err(handle_error(res).await)
    }
}

pub async fn remove_favorite(user_id: i64, product_id: i64) -> Result<(), String> {
    let client = Client::new();
    let res = client
        .delete(&get_url(&format!("/favorites/{}?user_id={}", product_id, user_id)))
        .send()
        .await
        .map_err(handle_network_error)?;

    if res.status().is_success() {
        Ok(())
    } else {
        Err(handle_error(res).await)
    }
}

pub async fn check_favorite(user_id: i64, product_id: i64) -> Result<bool, String> {
    let client = Client::new();
    let res = client
        .get(&get_url(&format!("/favorites/check?user_id={}&product_id={}", user_id, product_id)))
        .send()
        .await
        .map_err(handle_network_error)?;

    if res.status().is_success() {
        let check_res = res.json::<FavoriteCheckResponse>().await.map_err(|e| format!("Error al verificar favorito: {}", e))?;
        Ok(check_res.is_favorite)
    } else {
        Err(handle_error(res).await)
    }
}
