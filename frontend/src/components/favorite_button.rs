use leptos::*;
use crate::api::favorites_api::{add_favorite, remove_favorite, check_favorite};

#[component]
pub fn FavoriteButton(product_id: i64) -> impl IntoView {
    let (is_fav, set_is_fav) = create_signal(false);
    let (is_logged_in, _) = create_signal(false); // we will get this from context or local storage
    
    // Attempt to load initial state
    create_effect(move |_| {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(id_str)) = storage.get_item("user_id") {
                    if let Ok(uid) = id_str.parse::<i64>() {
                        spawn_local(async move {
                            if let Ok(status) = check_favorite(uid, product_id).await {
                                set_is_fav.set(status);
                            }
                        });
                    }
                }
            }
        }
    });

    let toggle_favorite = move |e: ev::MouseEvent| {
        e.prevent_default();
        e.stop_propagation();

        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(id_str)) = storage.get_item("user_id") {
                    if let Ok(uid) = id_str.parse::<i64>() {
                        let current = is_fav.get();
                        spawn_local(async move {
                            if current {
                                if let Ok(_) = remove_favorite(uid, product_id).await {
                                    set_is_fav.set(false);
                                }
                            } else {
                                if let Ok(_) = add_favorite(uid, product_id).await {
                                    set_is_fav.set(true);
                                }
                            }
                        });
                        return;
                    }
                }
            }
            // If not logged in
            if let Some(w) = web_sys::window() {
                let _ = w.alert_with_message("Inicia sesión para guardar favoritos");
            }
        }
    };

    view! {
        <button
            on:click=toggle_favorite
            class="favorite-btn"
            style="background: transparent; border: none; cursor: pointer; padding: 5px; display: inline-flex; align-items: center; justify-content: center;"
            title="Añadir a favoritos"
        >
            <svg 
                xmlns="http://www.w3.org/2000/svg" 
                viewBox="0 0 24 24" 
                width="24" 
                height="24" 
                fill=move || if is_fav.get() { "#ef4444" } else { "none" }
                stroke=move || if is_fav.get() { "#ef4444" } else { "#6b7280" }
                stroke-width="2" 
                stroke-linecap="round" 
                stroke-linejoin="round"
                style="transition: all 0.2s ease;"
            >
                <path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"></path>
            </svg>
        </button>
    }
}
