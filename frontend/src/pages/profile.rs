use leptos::*;

use crate::api::users_api;

#[component]
pub fn Profile() -> impl IntoView {
    let user_id = move || {
        let mut id_val = 0;
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(id_str)) = storage.get_item("user_id") {
                    id_val = id_str.parse::<i64>().unwrap_or(0);
                }
            }
        }
        id_val
    };

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
        <div class="profile-container">
            <h2>"Perfil de Usuario"</h2>
            <Suspense fallback=move || view! { <p>"Cargando perfil..."</p> }>
                {move || {
                    user_resource.get().flatten().map(|user| view! {
                        <div class="profile-card">
                            <h3>{user.name.clone()}</h3>
                            <p><strong>"Email: "</strong> {user.email.clone()}</p>
                            <p><strong>"Teléfono: "</strong> {user.phone.clone().unwrap_or("No especificado".into())}</p>
                            <p><strong>"Ubicación: "</strong> {user.location.clone().unwrap_or("No especificada".into())}</p>
                            <a href=format!("/users/{}/products", user.id) class="btn-secondary">"Ver productos publicados"</a>
                        </div>
                    }.into_view()).unwrap_or_else(|| view! {
                        <div class="error-msg">"Usuario no encontrado o error al cargar el perfil."</div>
                    }.into_view())
                }}
            </Suspense>
        </div>
    }
}
