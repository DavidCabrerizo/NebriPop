use leptos::*;
use leptos_router::*;

use crate::pages::{
    create_product::CreateProduct, edit_product::EditProduct, home::Home, product_detail::ProductDetail,
    login::Login, register::Register, profile::Profile, user_products::UserProducts,
    favorites::Favorites, messages::Messages, conversation::Conversation,
    contact::Contact,
};

#[component]
pub fn App() -> impl IntoView {
    let (user_name, set_user_name) = create_signal(String::new());
    let (is_logged_in, set_is_logged_in) = create_signal(false);
    let (unread_count, set_unread_count) = create_signal(0i64);
    let (is_dark_mode, set_is_dark_mode) = create_signal(false);

    // Load theme preference on mount
    create_effect(move |_| {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(theme)) = storage.get_item("theme_preference") {
                    set_is_dark_mode.set(theme == "dark");
                }
            }
        }
    });

    // Apply theme changes to document and save preference
    create_effect(move |_| {
        let is_dark = is_dark_mode.get();
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(element) = document.document_element() {
                    if is_dark {
                        let _ = element.set_attribute("data-theme", "dark");
                    } else {
                        let _ = element.remove_attribute("data-theme");
                    }
                }
            }
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("theme_preference", if is_dark { "dark" } else { "light" });
            }
        }
    });

    provide_context(user_name);
    provide_context(set_user_name);
    provide_context(is_logged_in);
    provide_context(set_is_logged_in);
    provide_context(unread_count);
    provide_context(set_unread_count);

    // Check login state on component mount
    create_effect(move |_| {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(name)) = storage.get_item("user_name") {
                    set_user_name.set(name);
                    set_is_logged_in.set(true);
                }
            }
        }
    });

    create_effect(move |_| {
        if is_logged_in.get() {
            let mut id_val = 0;
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(id_str)) = storage.get_item("user_id") {
                        id_val = id_str.parse::<i64>().unwrap_or(0);
                    }
                }
            }
            if id_val > 0 {
                spawn_local(async move {
                    if let Ok(count) = crate::api::messages_api::get_unread_count(id_val).await {
                        set_unread_count.set(count);
                    }
                });
            }
        } else {
            set_unread_count.set(0);
        }
    });

    let logout = move |_| {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.remove_item("user_id");
                let _ = storage.remove_item("user_name");
                set_is_logged_in.set(false);
                set_user_name.set(String::new());
            }
        }
    };

    view! {
        <Router>
            <div style="display: flex; flex-direction: column; min-height: 100vh;">
                <header>
                <div class="container" style="display: flex; justify-content: space-between; align-items: center; width: 100%;">
                    <A href="/" class="brand">
                        <div style="display: flex; align-items: center; gap: 10px;">
                            <img src="/logo.png?v=2" alt="NebriPop Logo" style="height: 40px;"/>
                        </div>
                    </A>
                    <div style="display: flex; gap: 15px; align-items: center; flex-wrap: wrap; justify-content: center;">
                        <button 
                            on:click=move |_| set_is_dark_mode.update(|d| *d = !*d) 
                            class="btn-secondary"
                            style="padding: 5px 10px; cursor: pointer; display: flex; align-items: center; justify-content: center;"
                            aria-label="Alternar modo oscuro"
                        >
                            {move || if is_dark_mode.get() { "☀️" } else { "🌙" }}
                        </button>
                        <A href="/products/new" class="btn">"+ Publicar Producto"</A>
                        <Show
                            when=move || is_logged_in.get()
                            fallback=|| view! {
                                <A href="/login" class="btn-secondary">"Iniciar Sesión"</A>
                            }
                        >
                            <A href="/profile" class="btn-secondary">
                                <span style="position: relative; display: inline-flex; align-items: center;">
                                    "Perfil (" {move || user_name.get()} ")"
                                    {move || {
                                        let count = unread_count.get();
                                        if count > 0 {
                                            view! { <span style="position: absolute; top: -12px; right: -20px; background: var(--accent-color); color: var(--text-color); border-radius: 50%; min-width: 20px; height: 20px; display: flex; align-items: center; justify-content: center; font-size: 0.75rem; font-weight: bold; border: 2px solid white; box-shadow: 0 2px 4px rgba(0,0,0,0.2);">{count}</span> }.into_view()
                                        } else {
                                            view! { <span></span> }.into_view()
                                        }
                                    }}
                                </span>
                            </A>
                            <button on:click=logout class="btn-danger" style="padding: 5px 10px; cursor: pointer;">"Salir"</button>
                        </Show>
                    </div>
                </div>
            </header>
            <main class="container" style="flex: 1; width: 100%; box-sizing: border-box;">
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/products/new" view=CreateProduct/>
                    <Route path="/products/:id" view=ProductDetail/>
                    <Route path="/products/:id/edit" view=EditProduct/>
                    <Route path="/login" view=Login/>
                    <Route path="/register" view=Register/>
                    <Route path="/profile" view=Profile/>
                    <Route path="/users/:id/products" view=UserProducts/>
                    <Route path="/favorites" view=Favorites/>
                    <Route path="/messages" view=Messages/>
                    <Route path="/products/:id/conversation/:other_id" view=Conversation/>
                    <Route path="/contact" view=Contact/>
                </Routes>
            </main>
            <footer class="site-footer">
                <div style="font-weight: 500; font-size: 1.1rem; display: flex; align-items: center; justify-content: center; gap: 8px;">
                    "NebriPop hecho con amor"
                    <span style="color: var(--danger-color); font-size: 1.2rem;">"❤️"</span>
                </div>
                <div style="margin-top: 15px;">
                    <A href="/contact" class="footer-link">"Contacto"</A>
                </div>
            </footer>
            </div>
        </Router>
    }
}
