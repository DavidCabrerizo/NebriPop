use leptos::*;
use leptos_router::*;

use crate::pages::{
    create_product::CreateProduct, home::Home, product_detail::ProductDetail,
    login::Login, register::Register, profile::Profile, user_products::UserProducts,
};

#[component]
pub fn App() -> impl IntoView {
    let (user_name, set_user_name) = create_signal(String::new());
    let (is_logged_in, set_is_logged_in) = create_signal(false);

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
            <header>
                <div class="container" style="display: flex; justify-content: space-between; align-items: center; width: 100%;">
                    <A href="/" class="brand">"NebriPop"</A>
                    <div style="display: flex; gap: 15px; align-items: center;">
                        <A href="/products/new" class="btn">"+ Publicar Producto"</A>
                        <Show
                            when=move || is_logged_in.get()
                            fallback=|| view! {
                                <A href="/login" class="btn-secondary">"Iniciar Sesión"</A>
                            }
                        >
                            <A href="/profile" class="btn-secondary">"Perfil (" {move || user_name.get()} ")"</A>
                            <button on:click=logout class="btn-danger" style="padding: 5px 10px; cursor: pointer;">"Salir"</button>
                        </Show>
                    </div>
                </div>
            </header>
            <main class="container">
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/products/new" view=CreateProduct/>
                    <Route path="/products/:id" view=ProductDetail/>
                    <Route path="/login" view=Login/>
                    <Route path="/register" view=Register/>
                    <Route path="/profile" view=Profile/>
                    <Route path="/users/:id/products" view=UserProducts/>
                </Routes>
            </main>
        </Router>
    }
}
