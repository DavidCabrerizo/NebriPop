use crate::api::categories_api::fetch_categories;
use crate::api::products_api::create_product;
use crate::components::error_message::ErrorMessage;
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
    let (image_url, set_image_url) = create_signal("".to_string());
    
    let (error_msg, set_error_msg) = create_signal(None::<String>);
    let (success_msg, set_success_msg) = create_signal(None::<String>);

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        set_error_msg(None);
        set_success_msg(None);

        let t = title.get();
        let d = description.get();
        let p = price.get();
        let cat = category_id.get();
        let cond = condition.get();
        let loc = location.get();
        
        if t.is_empty() || d.is_empty() || loc.is_empty() {
            set_error_msg(Some("Por favor, completa todos los campos obligatorios.".to_string()));
            return;
        }
        
        if p < 0.0 {
            set_error_msg(Some("El precio no puede ser negativo.".to_string()));
            return;
        }

        if cat == 0 {
             set_error_msg(Some("Por favor, selecciona una categoría.".to_string()));
             return;
        }

        let img = image_url.get();
        let main_image_url = if img.is_empty() { None } else { Some(img) };

        let dto = CreateProductDto {
            user_id: 1, // TEMPORAL PARA MVP (hasta implementar login)
            category_id: cat,
            title: t,
            description: d,
            price: p,
            condition: cond,
            location: loc,
            main_image_url,
        };

        spawn_local(async move {
            match create_product(dto).await {
                Ok(_) => {
                    set_success_msg(Some("¡Producto publicado correctamente!".to_string()));
                    // Limpiar formulario
                    set_title("".to_string());
                    set_description("".to_string());
                    set_price(0.0);
                    set_location("".to_string());
                    set_image_url("".to_string());
                    
                    // Pequeña pausa y redirigir
                    let navigate = use_navigate();
                    navigate("/", Default::default());
                }
                Err(e) => {
                    set_error_msg(Some(e));
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
                    <input type="text" prop:value=title on:input=move |ev| set_title(event_target_value(&ev)) required/>
                </div>
                
                <div class="form-group">
                    <label>"Categoría *"</label>
                    <select prop:value=move || category_id.get().to_string() on:change=move |ev| {
                        if let Ok(val) = event_target_value(&ev).parse::<i64>() {
                            set_category_id(val);
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
                            set_price(val);
                        }
                    } required/>
                </div>
                
                <div class="form-group">
                    <label>"Estado *"</label>
                    <select prop:value=condition on:change=move |ev| set_condition(event_target_value(&ev)) required>
                        <option value="new">"Nuevo"</option>
                        <option value="like_new">"Como nuevo"</option>
                        <option value="good">"Bueno"</option>
                        <option value="used">"Usado"</option>
                        <option value="damaged">"Dañado"</option>
                    </select>
                </div>
                
                <div class="form-group">
                    <label>"Ubicación *"</label>
                    <input type="text" prop:value=location on:input=move |ev| set_location(event_target_value(&ev)) required/>
                </div>
                
                <div class="form-group">
                    <label>"URL de la Imagen (opcional)"</label>
                    <input type="url" prop:value=image_url on:input=move |ev| set_image_url(event_target_value(&ev))/>
                </div>
                
                <div class="form-group">
                    <label>"Descripción *"</label>
                    <textarea prop:value=description on:input=move |ev| set_description(event_target_value(&ev)) required></textarea>
                </div>
                
                <button type="submit" class="btn" style="width: 100%; font-size: 1.1rem; padding: 12px;">"Publicar Producto"</button>
                <div style="text-align: center; margin-top: 15px;">
                    <A href="/" style="color: #6b7280; text-decoration: none;">"Cancelar"</A>
                </div>
            </form>
        </div>
    }
}
