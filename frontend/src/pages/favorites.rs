use leptos::*;
use crate::api::favorites_api::get_user_favorites;
use crate::components::product_card::ProductCard;
use crate::components::loading::Loading;
use crate::models::product::Product;

#[component]
pub fn Favorites() -> impl IntoView {
    let (user_id, set_user_id) = create_signal(0i64);

    create_effect(move |_| {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(id_str)) = storage.get_item("user_id") {
                    if let Ok(uid) = id_str.parse::<i64>() {
                        set_user_id.set(uid);
                    }
                }
            }
        }
    });

    let favorites_resource = create_resource(
        move || user_id.get(),
        |uid| async move {
            if uid == 0 {
                return Ok(vec![]);
            }
            get_user_favorites(uid).await
        },
    );

    view! {
        <div class="favorites-page">
            <h1 style="margin-bottom: 1rem; color: #111827;">"Mis Favoritos"</h1>
            
            {move || {
                if user_id.get() == 0 {
                    return view! {
                        <div class="empty-state" style="text-align: center; padding: 40px; background: white; border-radius: 8px; box-shadow: 0 1px 3px rgba(0,0,0,0.1);">
                            <p style="color: #6b7280; font-size: 1.1rem;">"Inicia sesión para ver tus favoritos."</p>
                        </div>
                    }.into_view();
                }

                view! {
                    <Transition fallback=move || view! { <Loading/> }>
                        {move || match favorites_resource.get() {
                            None => view! { <Loading/> }.into_view(),
                            Some(Ok(products)) if products.is_empty() => view! {
                                <div class="empty-state" style="text-align: center; padding: 40px; background: white; border-radius: 8px; box-shadow: 0 1px 3px rgba(0,0,0,0.1);">
                                    <p style="color: #6b7280; font-size: 1.1rem;">"No tienes productos favoritos todavía."</p>
                                </div>
                            }.into_view(),
                            Some(Ok(products)) => view! {
                                <div class="products-grid" style="display: grid; grid-template-columns: repeat(auto-fill, minmax(250px, 1fr)); gap: 1.5rem;">
                                    {products.into_iter().map(|product| view! {
                                        <ProductCard product=product />
                                    }).collect_view()}
                                </div>
                            }.into_view(),
                            Some(Err(e)) => view! {
                                <div class="error" style="color: #ef4444; padding: 20px; background: #fee2e2; border-radius: 8px;">
                                    {e}
                                </div>
                            }.into_view(),
                        }}
                    </Transition>
                }.into_view()
            }}
        </div>
    }
}
