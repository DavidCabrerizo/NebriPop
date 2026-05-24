use crate::api::products_api::fetch_products;
use crate::components::error_message::ErrorMessage;
use crate::components::loading::Loading;
use crate::components::product_card::ProductCard;
use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    let products = create_resource(|| (), |_| async move { fetch_products().await });

    view! {
        <div>
            <h2>"Últimos productos"</h2>
            <Suspense fallback=move || view! { <Loading/> }>
                {move || {
                    products.get().map(|result| match result {
                        Ok(data) => {
                            if data.is_empty() {
                                view! { <p>"No hay productos disponibles por el momento."</p> }.into_view()
                            } else {
                                view! {
                                    <div class="grid">
                                        {data.into_iter().map(|p| view! { <ProductCard product=p/> }).collect_view()}
                                    </div>
                                }.into_view()
                            }
                        }
                        Err(e) => view! { <ErrorMessage message=e/> }.into_view()
                    })
                }}
            </Suspense>
        </div>
    }
}
