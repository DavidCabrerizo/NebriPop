use leptos::*;

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <div style="max-width: 600px; margin: 40px auto; padding: 30px; background: white; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1);">
            <h2 style="color: var(--primary-color); text-align: center; margin-bottom: 20px;">"Contacto"</h2>
            <p style="text-align: center; margin-bottom: 30px;">"¿Tienes alguna duda o sugerencia? Envíanos un correo a visualproductor@gmail.com o usa el formulario a continuación (abrirá tu cliente de correo)."</p>
            
            <form action="mailto:visualproductor@gmail.com" method="GET" enctype="text/plain" style="display: flex; flex-direction: column; gap: 15px;">
                <div class="form-group">
                    <label style="font-weight: 600;">"Nombre / Asunto"</label>
                    <input type="text" name="subject" placeholder="Ej: Juan - Sugerencia" required=true style="width: 100%; padding: 10px; border: 1px solid var(--border-color); border-radius: 8px; font-family: inherit;" />
                </div>
                <div class="form-group">
                    <label style="font-weight: 600;">"Mensaje"</label>
                    <textarea name="body" placeholder="Escribe aquí tu mensaje..." rows="6" required=true style="width: 100%; padding: 10px; border: 1px solid var(--border-color); border-radius: 8px; font-family: inherit; resize: vertical;"></textarea>
                </div>
                <button type="submit" class="btn btn-primary" style="width: 100%;">"Enviar Mensaje"</button>
            </form>
        </div>
    }
}
