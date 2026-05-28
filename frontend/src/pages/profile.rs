use leptos::*;
use leptos_router::*;

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

    let (is_editing, set_is_editing) = create_signal(false);
    let (edit_phone, set_edit_phone) = create_signal("".to_string());
    let (edit_location, set_edit_location) = create_signal("".to_string());
    let (save_error, set_save_error) = create_signal(None::<String>);
    let (save_success, set_save_success) = create_signal(None::<String>);

    let start_edit = move |phone: Option<String>, location: Option<String>| {
        set_edit_phone.set(phone.unwrap_or_default());
        set_edit_location.set(location.unwrap_or_default());
        set_is_editing.set(true);
        set_save_error.set(None);
        set_save_success.set(None);
    };

    let cancel_edit = move |_| {
        set_is_editing.set(false);
    };

    let save_edit = move |_| {
        let u_id = user_id();
        let phone = edit_phone.get();
        let location = edit_location.get();
        
        spawn_local(async move {
            let p_opt = if phone.is_empty() { None } else { Some(phone.as_str()) };
            let l_opt = if location.is_empty() { None } else { Some(location.as_str()) };
            
            match users_api::update_user_profile(u_id, p_opt, l_opt).await {
                Ok(_) => {
                    set_save_success.set(Some("Perfil actualizado correctamente".into()));
                    set_is_editing.set(false);
                    user_resource.refetch(); // Refrescar los datos
                }
                Err(e) => {
                    set_save_error.set(Some(e));
                }
            }
        });
    };

    view! {
        <div class="profile-container">
            <h2>"Perfil de Usuario"</h2>
            <Suspense fallback=move || view! { <p>"Cargando perfil..."</p> }>
                {move || {
                    user_resource.get().flatten().map(|user| {
                        let phone_clone = user.phone.clone();
                        let location_clone = user.location.clone();
                        let phone_clone2 = phone_clone.clone();
                        let location_clone2 = location_clone.clone();
                        let on_click_edit = move |_| start_edit(phone_clone2.clone(), location_clone2.clone());
                        
                        view! {
                        <div class="profile-card">
                            {move || if let Some(err) = save_error.get() {
                                view! { <div class="error-msg">{err}</div> }.into_view()
                            } else {
                                view! { <div></div> }.into_view()
                            }}
                            {move || if let Some(msg) = save_success.get() {
                                view! { <div class="success-msg">{msg}</div> }.into_view()
                            } else {
                                view! { <div></div> }.into_view()
                            }}

                            {move || if is_editing.get() {
                                view! {
                                    <div class="form-group">
                                        <label>"Teléfono:"</label>
                                        <input type="text" prop:value=edit_phone on:input=move |ev| set_edit_phone.set(event_target_value(&ev)) />
                                    </div>
                                    <div class="form-group">
                                        <label>"Ubicación:"</label>
                                        <input type="text" prop:value=edit_location on:input=move |ev| set_edit_location.set(event_target_value(&ev)) />
                                    </div>
                                    <div style="display: flex; gap: 10px; margin-top: 15px;">
                                        <button class="btn btn-primary" on:click=save_edit>"Guardar Cambios"</button>
                                        <button class="btn btn-secondary" on:click=cancel_edit>"Cancelar"</button>
                                    </div>
                                }.into_view()
                            } else {
                                view! {
                                    <h3>{user.name.clone()}</h3>
                                    <p><strong>"Email: "</strong> {user.email.clone()}</p>
                                    <p><strong>"Teléfono: "</strong> {user.phone.clone().unwrap_or("No especificado".into())}</p>
                                    <p><strong>"Ubicación: "</strong> {user.location.clone().unwrap_or("No especificada".into())}</p>
                                    
                                    <div style="display: flex; gap: 10px; margin-top: 15px; flex-wrap: wrap;">
                                        <A href=format!("/users/{}/products", user.id) class="btn btn-secondary">"Ver productos publicados"</A>
                                        <A href="/favorites" class="btn btn-secondary">"Mis Favoritos"</A>
                                        <button class="btn btn-primary" on:click=on_click_edit.clone()>"Editar Perfil"</button>
                                    </div>
                                }.into_view()
                            }}
                        </div>
                    }.into_view()
                    }).unwrap_or_else(|| {
                        view! { <div class="error-msg">"Usuario no encontrado o error al cargar el perfil."</div> }.into_view()
                    })
                }}
            </Suspense>
        </div>
    }
}
