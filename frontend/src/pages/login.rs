use leptos::*;
use leptos_router::*;
use crate::api::auth_api;

#[component]
pub fn Login() -> impl IntoView {
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (error_msg, set_error_msg) = create_signal(Option::<String>::None);
    let (show_password, set_show_password) = create_signal(false);
    
    let on_recover_password = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        let e = email.get();
        if e.is_empty() {
            if let Some(window) = web_sys::window() {
                let _ = window.alert_with_message("Por favor, introduce tu correo electrónico en el campo superior para recuperar la contraseña.");
            }
        } else {
            if let Some(window) = web_sys::window() {
                let _ = window.alert_with_message(&format!("(Simulación MVP) Se ha enviado un enlace de recuperación a: {}", e));
            }
        }
    };
    
    let login_action = create_action(move |(e, p): &(String, String)| {
        let e = e.clone();
        let p = p.clone();
        async move {
            match auth_api::login(&e, &p).await {
                Ok(res) => {
                    // Guardar temporalmente en localStorage
                    if let Some(window) = web_sys::window() {
                        if let Ok(Some(storage)) = window.local_storage() {
                            let _ = storage.set_item("user_id", &res.user.id.to_string());
                            let _ = storage.set_item("user_name", &res.user.name);
                        }
                    }
                    if let Some(set_name) = use_context::<WriteSignal<String>>() {
                        set_name.set(res.user.name.clone());
                    }
                    if let Some(set_logged_in) = use_context::<WriteSignal<bool>>() {
                        set_logged_in.set(true);
                    }
                    let navigate = use_navigate();
                    navigate("/", Default::default());
                }
                Err(err) => {
                    set_error_msg.set(Some(err));
                }
            }
        }
    });

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        login_action.dispatch((email.get(), password.get()));
    };

    view! {
        <div class="auth-container">
            <h2>"Iniciar Sesión"</h2>
            {move || error_msg.get().map(|msg| view! { <div class="error-msg">{msg}</div> })}
            <form on:submit=on_submit class="auth-form">
                <div class="form-group">
                    <label>"Email"</label>
                    <input 
                        type="email" 
                        required 
                        on:input=move |ev| set_email.set(event_target_value(&ev)) 
                        prop:value=email 
                    />
                </div>
                <div class="form-group password-group">
                    <label>"Contraseña"</label>
                    <div style="display: flex; gap: 8px;">
                        <input 
                            type=move || if show_password.get() { "text" } else { "password" }
                            required 
                            on:input=move |ev| set_password.set(event_target_value(&ev)) 
                            prop:value=password 
                            style="flex: 1;"
                        />
                        <button type="button" class="btn-secondary" style="padding: 0 10px;" on:click=move |_| set_show_password.update(|v| *v = !*v)>
                            {move || if show_password.get() { "Ocultar" } else { "Ver" }}
                        </button>
                    </div>
                </div>
                <div style="text-align: right; margin-bottom: 15px; font-size: 0.9em;">
                    <a href="#" on:click=on_recover_password>"¿Has olvidado tu contraseña?"</a>
                </div>
                <button type="submit" class="btn-primary" disabled=move || login_action.pending().get()>"Entrar"</button>
            </form>
            <p>"¿No tienes cuenta? " <a href="/register">"Regístrate"</a></p>
        </div>
    }
}
