use crate::api::products_api::fetch_product_by_id;
use crate::components::error_message::ErrorMessage;
use crate::components::loading::Loading;
use leptos::*;
use leptos_router::*;

#[derive(Params, PartialEq, Clone)]
struct ProductParams {
    id: Option<String>,
}

#[component]
pub fn ProductDetail() -> impl IntoView {
    let params = use_params::<ProductParams>();
    let (active_image, set_active_image) = create_signal(None::<String>);
    
    let product_id = move || {
        params.with(|params_res| {
            params_res
                .as_ref()
                .ok()
                .and_then(|p| p.id.clone())
                .and_then(|id_str| id_str.parse::<i64>().ok())
                .unwrap_or(0)
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
                        Ok(detail_response) => {
                            let product = detail_response.product;
                            let author_name = detail_response.author_name;
                            let img_src_raw = product.main_image_url.clone().unwrap_or_default();
                            let img_src = if img_src_raw.is_empty() {
                                "https://via.placeholder.com/600x400?text=Sin+imagen".to_string()
                            } else if img_src_raw.starts_with("http") {
                                img_src_raw
                            } else {
                                format!("http://127.0.0.1:3000/{}", img_src_raw)
                            };

                            let thumbnails = product.images.clone().unwrap_or_default();

                            view! {
                                <div class="detail-view">
                                    <div class="detail-image-container">
                                        <img src=move || active_image.get().unwrap_or_else(|| img_src.clone()) alt=product.title.clone() class="detail-img"/>
                                        {if !thumbnails.is_empty() {
                                            view! {
                                                <div class="thumbnails" style="display: flex; gap: 10px; margin-top: 10px; overflow-x: auto;">
                                                    {thumbnails.into_iter().map(|img| {
                                                        let t_src = if img.image_url.starts_with("http") {
                                                            img.image_url.clone()
                                                        } else {
                                                            format!("http://127.0.0.1:3000/{}", img.image_url)
                                                        };
                                                        let t_src_clone = t_src.clone();
                                                        view! {
                                                            <img 
                                                                src=t_src 
                                                                alt="thumbnail" 
                                                                style="width: 80px; height: 80px; object-fit: cover; border-radius: 4px; cursor: pointer;"
                                                                on:click=move |_| set_active_image.set(Some(t_src_clone.clone()))
                                                            />
                                                        }
                                                    }).collect_view()}
                                                </div>
                                            }.into_view()
                                        } else {
                                            view! { <div></div> }.into_view()
                                        }}
                                    </div>
                                    <div class="detail-content">
                                        <h1 style="margin-top:0;">{product.title}</h1>
                                        <h2 style="color: var(--primary-color);">{format!("{} €", product.price)}</h2>
                                        
                                        <div style="margin: 20px 0; padding: 15px; background: #f3f4f6; border-radius: 6px;">
                                            <p><strong>"Estado: "</strong> {product.condition}</p>
                                            <p><strong>"Ubicación: "</strong> {product.location}</p>
                                            <p><strong>"Publicado el: "</strong> {product.created_at}</p>
                                            <p><strong>"Publicado por: "</strong> {author_name}</p>
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
