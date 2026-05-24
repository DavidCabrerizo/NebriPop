use crate::api::products_api::fetch_product_by_id;
use crate::components::error_message::ErrorMessage;
use crate::components::loading::Loading;
use leptos::*;
use leptos_router::*;

#[derive(Params, PartialEq, Clone)]
struct ProductParams {
    id: i64,
}

#[component]
pub fn ProductDetail() -> impl IntoView {
    let params = use_params::<ProductParams>();
    
    let product_id = move || {
        params.with(|p| {
            p.as_ref().map(|p| p.id).unwrap_or(0)
        })
    };

    let product_resource = create_resource(product_id, |id| async move {
        if id == 0 {
            return Err("ID de producto inválido".to_string());
        }
        fetch_product_by_id(id).await
    });

    view! {
        <div>
            <div style="margin-bottom: 20px;">
                <A href="/" class="btn btn-secondary">"← Volver al listado"</A>
            </div>
            
            <Suspense fallback=move || view! { <Loading/> }>
                {move || {
                    product_resource.get().map(|result| match result {
                        Ok(product) => {
                            let img_src = product.main_image_url.clone().unwrap_or_else(|| "https://via.placeholder.com/600x400".to_string());
                            view! {
                                <div class="detail-view">
                                    <img src=img_src alt=product.title.clone() class="detail-img"/>
                                    <div class="detail-content">
                                        <h1 style="margin-top:0;">{product.title}</h1>
                                        <h2 style="color: var(--primary-color);">{format!("{} €", product.price)}</h2>
                                        
                                        <div style="margin: 20px 0; padding: 15px; background: #f3f4f6; border-radius: 6px;">
                                            <p><strong>"Estado: "</strong> {product.condition}</p>
                                            <p><strong>"Ubicación: "</strong> {product.location}</p>
                                            <p><strong>"Publicado el: "</strong> {product.created_at}</p>
                                        </div>
                                        
                                        <h3>"Descripción"</h3>
                                        <p style="white-space: pre-wrap;">{product.description}</p>
                                    </div>
                                </div>
                            }.into_view()
                        }
                        Err(e) => view! { <ErrorMessage message=e/> }.into_view()
                    })
                }}
            </Suspense>
        </div>
    }
}
