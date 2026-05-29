use leptos::*;
use leptos_router::*;
use crate::api::users_api;

#[component]
pub fn UserProducts() -> impl IntoView {
    let params = use_params_map();
    let user_id_str = move || params.with(|p| p.get("id").cloned().unwrap_or_default());
    let user_id = move || user_id_str().parse::<i64>().unwrap_or(0);

    let products_resource = create_resource(
        user_id,
        |id| async move {
            if id > 0 {
                users_api::get_user_products(id).await.unwrap_or_default()
            } else {
                vec![]
            }
        }
    );

    let user_resource = create_resource(
        user_id,
        |id| async move {
            if id > 0 {
                users_api::get_user_profile(id).await.ok()
            } else {
                None
            }
        }
    );

    view! {
        <div class="user-products-container">
            <h2>"Productos del Usuario " {move || user_resource.get().flatten().map(|u| u.name).unwrap_or_default()}</h2>
            <Suspense fallback=move || view! { <p>"Cargando productos..."</p> }>
                <div class="products-grid">
                {move || {
                    products_resource.get().map(|products| {
                        if products.is_empty() {
                            view! { <p>"Este usuario no tiene productos publicados."</p> }.into_view()
                        } else {
                            products.into_iter().map(|p| {
                                let img_src_raw = p.main_image_url.clone().unwrap_or_default();
                                let img_src = if img_src_raw.is_empty() {
                                    "https://via.placeholder.com/300x200?text=Sin+imagen".to_string()
                                } else if img_src_raw.starts_with("http") {
                                    img_src_raw
                                } else {
                                    format!("http://127.0.0.1:3000/{}", img_src_raw)
                                };

                                let display_condition = match p.condition.as_str() {
                                    "new" => "Nuevo",
                                    "like_new" => "Como nuevo",
                                    "good" => "Bueno",
                                    "used" => "Usado",
                                    "damaged" => "Dañado",
                                    other => other,
                                }.to_string();

                                view! {
                                    <a href=format!("/products/{}", p.id) class="card" style="text-decoration: none; color: inherit;">
                                        <img src=img_src alt=p.title.clone() class="card-img"/>
                                        <div class="card-content">
                                            <h3 class="card-title">{p.title}</h3>
                                            <p class="card-price">{format!("{} €", p.price)}</p>
                                            <p class="card-meta">
                                                <strong>"Estado: "</strong> {display_condition}
                                            </p>
                                        </div>
                                    </a>
                                }
                            }).collect_view()
                        }
                    }).unwrap_or_else(|| view! { <p>"Error al cargar los productos."</p> }.into_view())
                }}
                </div>
            </Suspense>
        </div>
    }
}
