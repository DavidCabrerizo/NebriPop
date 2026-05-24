use crate::models::product::Product;
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

    view! {
        <A href=detail_url class="card">
            <img src=img_src alt=product.title.clone() class="card-img" />
            <div class="card-content">
                <div class="card-price">{format!("{} €", product.price)}</div>
                <h3 class="card-title">{product.title}</h3>
                <div class="card-meta">
                    {product.location} " • " {product.condition}
                </div>
            </div>
        </A>
    }
}
