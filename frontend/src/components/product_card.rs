use crate::models::product::Product;
use crate::components::favorite_button::FavoriteButton;
use leptos::*;
use leptos_router::A;

#[component]
pub fn ProductCard(product: Product) -> impl IntoView {
    let img_src = product.main_image_url.clone().unwrap_or_default();
    let img_src = if img_src.is_empty() {
        "https://via.placeholder.com/300?text=Sin+imagen".to_string()
    } else if img_src.starts_with("http") {
        img_src
    } else {
        format!("http://127.0.0.1:3000/{}", img_src)
    };

    let detail_url = format!("/products/{}", product.id);

    let display_condition = match product.condition.as_str() {
        "new" => "Nuevo",
        "like_new" => "Como nuevo",
        "good" => "Bueno",
        "used" => "Usado",
        "damaged" => "Dañado",
        other => other,
    }.to_string();

    let display_status = match product.status.as_str() {
        "available" => "Disponible",
        "reserved" => "Reservado",
        "sold" => "Vendido",
        other => other,
    }.to_string();

    view! {
        <div style="position: relative; height: 100%;">
            <div style="position: absolute; top: 10px; right: 10px; z-index: 10; background: rgba(255,255,255,0.9); border-radius: 50%; box-shadow: 0 1px 3px rgba(0,0,0,0.2);">
                <FavoriteButton product_id=product.id />
            </div>
            <A href=detail_url class="card">
                <img src=img_src alt=product.title.clone() class="card-img" />
                <div class="card-content">
                    <div class="card-price">{format!("{} €", product.price)}</div>
                    <h3 class="card-title" style="margin: 0;">{product.title}</h3>
                    <div class="card-meta">
                        {product.location} " • " {display_condition} " • " {display_status}
                    </div>
                </div>
            </A>
        </div>
    }
}
