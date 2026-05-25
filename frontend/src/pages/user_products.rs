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

    view! {
        <div class="user-products-container">
            <h2>"Productos del Usuario"</h2>
            <Suspense fallback=move || view! { <p>"Cargando productos..."</p> }>
                <div class="products-grid">
                {move || {
                    products_resource.get().map(|products| {
                        if products.is_empty() {
                            view! { <p>"Este usuario no tiene productos publicados."</p> }.into_view()
                        } else {
                            products.into_iter().map(|p| view! {
                                <div class="product-card">
                                    <h3>{p.title}</h3>
                                    <p class="price">{format!("{} €", p.price)}</p>
                                    <p class="condition">{p.condition}</p>
                                    <a href=format!("/products/{}", p.id) class="btn-primary">"Ver Detalle"</a>
                                </div>
                            }).collect_view()
                        }
                    }).unwrap_or_else(|| view! { <p>"Error al cargar los productos."</p> }.into_view())
                }}
                </div>
            </Suspense>
        </div>
    }
}
