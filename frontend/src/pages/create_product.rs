use crate::api::categories_api::fetch_categories;
use crate::api::products_api::{create_product, upload_product_images};
use crate::models::product::CreateProductDto;
use leptos::*;
use leptos_router::*;

#[component]
pub fn CreateProduct() -> impl IntoView {
    let categories_resource = create_resource(|| (), |_| async move { fetch_categories().await });

    let (title, set_title) = create_signal("".to_string());
    let (description, set_description) = create_signal("".to_string());
    let (price, set_price) = create_signal(0.0);
    let (category_id, set_category_id) = create_signal(0i64);
    let (condition, set_condition) = create_signal("good".to_string());
    let (location, set_location) = create_signal("".to_string());
    let (status, set_status) = create_signal("available".to_string());
    let (image_url, set_image_url) = create_signal("".to_string());
    
    let main_file_input_ref = create_node_ref::<html::Input>();
    let secondary_file_input_ref = create_node_ref::<html::Input>();
    
    let (error_msg, set_error_msg) = create_signal(None::<String>);
    let (success_msg, set_success_msg) = create_signal(None::<String>);

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        set_error_msg.set(None);
        set_success_msg.set(None);

        let t = title.get();
        let d = description.get();
        let p = price.get();
        let cat = category_id.get();
        let cond = condition.get();
        let loc = location.get();
        let stat = status.get();
        
        if t.is_empty() || d.is_empty() || loc.is_empty() {
            set_error_msg.set(Some("Por favor, completa todos los campos obligatorios.".to_string()));
            return;
        }
        
        if p < 0.0 {
            set_error_msg.set(Some("El precio no puede ser negativo.".to_string()));
            return;
        }

        if cat == 0 {
             set_error_msg.set(Some("Por favor, selecciona una categoría.".to_string()));
             return;
        }

        let img = image_url.get();
        // Extract files from input
        let mut upload_files = Vec::new();
        
        // 1. Añadir primero la imagen principal (así el backend la guarda con position=0)
        if let Some(input) = main_file_input_ref.get() {
            if let Some(files) = input.files() {
                if let Some(file) = files.item(0) {
                    upload_files.push(file);
                }
            }
        }
        
        // 2. Añadir las imágenes secundarias
        if let Some(input) = secondary_file_input_ref.get() {
            if let Some(files) = input.files() {
                for i in 0..files.length() {
                    if let Some(file) = files.item(i) {
                        upload_files.push(file);
                    }
                }
            }
        }

        // GET USER ID FROM LOCALSTORAGE
        let mut user_id = 0;
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(id_str)) = storage.get_item("user_id") {
                    user_id = id_str.parse::<i64>().unwrap_or(0);
                }
            }
        }

        if user_id == 0 {
            set_error_msg.set(Some("Debes iniciar sesión para publicar un producto.".to_string()));
            return;
        }

        let dto = CreateProductDto {
            user_id,
            category_id: cat,
            title: t,
            description: d,
            price: p,
            condition: cond,
            location: loc,
            status: stat,
            main_image_url: None, // Will be set by backend upon image upload
        };

        spawn_local(async move {
            match create_product(dto).await {
                Ok(product) => {
                    // Si hay imágenes o URL, subirlas
                    let has_images = !upload_files.is_empty();
                    let has_url = !img.is_empty();
                    
                    if has_images || has_url {
                        let url_opt = if has_url { Some(img) } else { None };
                        match upload_product_images(product.id, upload_files, url_opt).await {
                            Ok(_) => {
                                set_success_msg.set(Some("¡Producto publicado con imágenes correctamente!".to_string()));
                            }
                            Err(e) => {
                                set_error_msg.set(Some(format!("Producto creado, pero falló la subida de imágenes: {}", e)));
                            }
                        }
                    } else {
                        set_success_msg.set(Some("¡Producto publicado correctamente!".to_string()));
                    }

                    if error_msg.get().is_none() {
                        let navigate = use_navigate();
                        navigate("/", Default::default());
                    }
                }
                Err(e) => {
                    set_error_msg.set(Some(e));
                }
            }
        });
    };

    view! {
        <div style="max-width: 600px; margin: 0 auto; background: white; padding: 20px; border-radius: 8px; box-shadow: 0 1px 3px rgba(0,0,0,0.1);">
            <h2>"Publicar Nuevo Producto"</h2>
            
            {move || error_msg.get().map(|msg| view! { <div class="alert-error">{msg}</div> })}
            {move || success_msg.get().map(|msg| view! { <div class="alert-success">{msg}</div> })}
            
            <form on:submit=on_submit>
                <div class="form-group">
                    <label>"Título *"</label>
                    <input type="text" prop:value=title on:input=move |ev| set_title.set(event_target_value(&ev)) required/>
                </div>
                
                <div class="form-group">
                    <label>"Categoría *"</label>
                    <select prop:value=move || category_id.get().to_string() on:change=move |ev| {
                        if let Ok(val) = event_target_value(&ev).parse::<i64>() {
                            set_category_id.set(val);
                        }
                    } required>
                        <option value="0">"Selecciona una categoría"</option>
                        <Suspense fallback=move || view! { <option>"Cargando..."</option> }>
                            {move || {
                                categories_resource.get().map(|res| match res {
                                    Ok(categories) => {
                                        categories.into_iter().map(|c| {
                                            view! { <option value=c.id.to_string()>{c.name}</option> }
                                        }).collect_view()
                                    }
                                    Err(_) => view! { <option>"Error al cargar"</option> }.into_view()
                                })
                            }}
                        </Suspense>
                    </select>
                </div>
                
                <div class="form-group">
                    <label>"Precio (€) *"</label>
                    <input type="number" step="0.01" min="0" prop:value=move || price.get().to_string() on:input=move |ev| {
                        if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                            set_price.set(val);
                        }
                    } required/>
                </div>
                
                <div class="form-group">
                    <label>"Estado *"</label>
                    <select prop:value=condition on:change=move |ev| set_condition.set(event_target_value(&ev)) required>
                        <option value="new">"Nuevo"</option>
                        <option value="like_new">"Como nuevo"</option>
                        <option value="good">"Bueno"</option>
                        <option value="used">"Usado"</option>
                        <option value="damaged">"Dañado"</option>
                    </select>
                </div>
                
                <div class="form-group">
                    <label>"Ubicación *"</label>
                    <input type="text" prop:value=location on:input=move |ev| set_location.set(event_target_value(&ev)) required/>
                </div>
                
                <div class="form-group">
                    <label>"Disponibilidad *"</label>
                    <select prop:value=status on:change=move |ev| set_status.set(event_target_value(&ev)) required>
                        <option value="available">"Disponible"</option>
                        <option value="reserved">"Reservado"</option>
                        <option value="sold">"Vendido"</option>
                    </select>
                </div>
                
                <div class="form-group" style="border: 1px solid var(--border-color); padding: 15px; border-radius: 6px; background-color: #f9fafb;">
                    <label style="color: var(--primary-color); font-weight: bold;">"Imagen Principal (Local)"</label>
                    <p style="font-size: 0.85rem; color: #6b7280; margin-top: 0; margin-bottom: 10px;">"Esta será la imagen de portada de tu producto."</p>
                    <input type="file" _ref=main_file_input_ref accept="image/jpeg,image/png"/>
                </div>

                <div class="form-group" style="border: 1px solid var(--border-color); padding: 15px; border-radius: 6px; background-color: #f9fafb;">
                    <label>"Imágenes Secundarias (Local)"</label>
                    <p style="font-size: 0.85rem; color: #6b7280; margin-top: 0; margin-bottom: 10px;">"Puedes seleccionar múltiples imágenes adicionales."</p>
                    <input type="file" _ref=secondary_file_input_ref accept="image/jpeg,image/png" multiple/>
                </div>
                
                <div class="form-group">
                    <label>"URL de la Imagen (externa, opcional)"</label>
                    <input type="url" prop:value=image_url on:input=move |ev| set_image_url.set(event_target_value(&ev)) placeholder="https://..."/>
                </div>
                
                <div class="form-group">
                    <label>"Descripción *"</label>
                    <textarea prop:value=description on:input=move |ev| set_description.set(event_target_value(&ev)) required></textarea>
                </div>
                
                <button type="submit" class="btn" style="width: 100%; font-size: 1.1rem; padding: 12px;">"Publicar Producto"</button>
                <div style="text-align: center; margin-top: 15px;">
                    <a href="/" style="color: #6b7280; text-decoration: none;">"Cancelar"</a>
                </div>
            </form>
        </div>
    }
}
