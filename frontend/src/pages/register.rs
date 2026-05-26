use leptos::*;
use crate::api::auth_api;

#[component]
pub fn Register() -> impl IntoView {
    let (name, set_name) = create_signal(String::new());
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (phone, set_phone) = create_signal(String::new());
    let (location, set_location) = create_signal(String::new());
    let (error_msg, set_error_msg) = create_signal(Option::<String>::None);
    let (success_msg, set_success_msg) = create_signal(Option::<String>::None);
    
    let register_action = create_action(move |(n, e, p, ph, l): &(String, String, String, String, String)| {
        let n = n.clone();
        let e = e.clone();
        let p = p.clone();
        let ph = if ph.is_empty() { None } else { Some(ph.clone()) };
        let l = if l.is_empty() { None } else { Some(l.clone()) };
        
        async move {
            match auth_api::register(&n, &e, &p, ph.as_deref(), l.as_deref()).await {
                Ok(_) => {
                    set_success_msg.set(Some("Registro exitoso. Ya puedes iniciar sesión.".into()));
                    set_error_msg.set(None);
                }
                Err(err) => {
                    set_error_msg.set(Some(err));
                    set_success_msg.set(None);
                }
            }
        }
    });

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        register_action.dispatch((name.get(), email.get(), password.get(), phone.get(), location.get()));
    };

    view! {
        <div class="auth-container">
            <h2>"Registro"</h2>
            {move || error_msg.get().map(|msg| view! { <div class="error-msg">{msg}</div> })}
            {move || success_msg.get().map(|msg| view! { <div class="success-msg">{msg}</div> })}
            <form on:submit=on_submit class="auth-form">
                <div class="form-group">
                    <label>"Nombre"</label>
                    <input type="text" required on:input=move |ev| set_name.set(event_target_value(&ev)) prop:value=name />
                </div>
                <div class="form-group">
                    <label>"Email"</label>
                    <input type="email" required on:input=move |ev| set_email.set(event_target_value(&ev)) prop:value=email />
                </div>
                <div class="form-group">
                    <label>"Contraseña"</label>
                    <input type="password" required on:input=move |ev| set_password.set(event_target_value(&ev)) prop:value=password />
                </div>
                <div class="form-group">
                    <label>"Teléfono (opcional)"</label>
                    <input type="text" on:input=move |ev| set_phone.set(event_target_value(&ev)) prop:value=phone />
                </div>
                <div class="form-group">
                    <label>"Ubicación (opcional)"</label>
                    <input type="text" on:input=move |ev| set_location.set(event_target_value(&ev)) prop:value=location />
                </div>
                <button type="submit" class="btn-primary" disabled=move || register_action.pending().get()>"Registrarse"</button>
            </form>
            <p>"¿Ya tienes cuenta? " <a href="/login">"Inicia sesión"</a></p>
        </div>
    }
}
