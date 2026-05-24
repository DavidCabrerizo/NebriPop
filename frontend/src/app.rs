use leptos::*;
use leptos_router::*;

use crate::pages::{
    create_product::CreateProduct, home::Home, product_detail::ProductDetail,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <header>
                <div class="container" style="display: flex; justify-content: space-between; align-items: center; width: 100%;">
                    <A href="/" class="brand">"NebriPop"</A>
                    <A href="/products/new" class="btn">"+ Publicar Producto"</A>
                </div>
            </header>
            <main class="container">
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/products/new" view=CreateProduct/>
                    <Route path="/products/:id" view=ProductDetail/>
                </Routes>
            </main>
        </Router>
    }
}
