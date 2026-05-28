use crate::api::categories_api::fetch_categories;
use crate::api::products_api::{fetch_product_by_id, update_product, upload_product_images, delete_product_image};
use crate::models::product::{UpdateProductDto, ProductImage};
use crate::components::loading::Loading;
use crate::components::error_message::ErrorMessage;
use leptos::*;
use leptos_router::*;

#[derive(Params, PartialEq, Clone)]
struct EditProductParams {
    id: Option<String>,
}

#[component]
pub fn EditProduct() -> impl IntoView {
    let params = use_params::<EditProductParams>();
    
    let product_id = move || {
        params.with(|params_res| {
            params_res
                .as_ref()
                .ok()
                .and_then(|p| p.id.clone())
                .and_then(|id_str| id_str.parse::<i64>().ok())
                .unwrap_or(0)
        })
    };

    let categories_resource = create_resource(|| (), |_| async move { fetch_categories().await });
    
    // We fetch the product details
    let product_resource = create_resource(product_id, |id| async move {
        if id == 0 {
            return Err("ID de producto inválido".to_string());
        }
        fetch_product_by_id(id).await
    });

    let (title, set_title) = create_signal("".to_string());
    let (description, set_description) = create_signal("".to_string());
    let (price, set_price) = create_signal(0.0);
    let (category_id, set_category_id) = create_signal(0i64);
    let (condition, set_condition) = create_signal("good".to_string());
    let (location, set_location) = create_signal("".to_string());
    let (status, set_status) = create_signal("available".to_string());
    let (image_url, set_image_url) = create_signal("".to_string());
    let (existing_images, set_existing_images) = create_signal(Vec::<ProductImage>::new());
    
    let (is_initialized, set_is_initialized) = create_signal(false);
    
    create_effect(move |_| {
        if let Some(Ok(detail)) = product_resource.get() {
            if !is_initialized.get() {
                set_title.set(detail.product.title);
                set_description.set(detail.product.description);
                set_price.set(detail.product.price);
                set_category_id.set(detail.product.category_id);
                set_condition.set(detail.product.condition);
                set_location.set(detail.product.location);
                set_status.set(detail.product.status);
                if let Some(images) = detail.product.images {
                    set_existing_images.set(images);
                }
                set_is_initialized.set(true);
            }
        }
    });
    
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
        let pid = product_id();
        
        if pid == 0 {
            set_error_msg.set(Some("ID de producto inválido".to_string()));
            return;
        }

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
        let mut upload_files = Vec::new();
        
        if let Some(input) = main_file_input_ref.get() {
            if let Some(files) = input.files() {
                if let Some(file) = files.item(0) {
                    upload_files.push(file);
                }
            }
        }
        
        if let Some(input) = secondary_file_input_ref.get() {
            if let Some(files) = input.files() {
                for i in 0..files.length() {
                    if let Some(file) = files.item(i) {
                        upload_files.push(file);
                    }
                }
            }
        }

        let mut user_id = 0;
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(id_str)) = storage.get_item("user_id") {
                    user_id = id_str.parse::<i64>().unwrap_or(0);
                }
            }
        }

        if user_id == 0 {
            set_error_msg.set(Some("Debes iniciar sesión para editar un producto.".to_string()));
            return;
        }

        let dto = UpdateProductDto {
            user_id,
            category_id: cat,
            title: t,
            description: d,
            price: p,
            condition: cond,
            location: loc,
            status: Some(stat),
            main_image_url: None, // Logic backend handles it or we could keep current if we want, but repo will overwrite if we pass None. Wait, backend will overwrite main_image_url with None if we pass None!
        };
        // We should pass the existing main_image_url if we are not changing it right now? Actually, backend update query does: main_image_url = ?
        // We might need to keep the existing one. Let's fix that in DTO creation
        let existing_main_img = if existing_images.get().is_empty() {
            None
        } else {
            Some(existing_images.get()[0].image_url.clone())
        };
        
        let final_dto = UpdateProductDto {
            main_image_url: existing_main_img,
            ..dto
        };

        spawn_local(async move {
            match update_product(pid, final_dto).await {
                Ok(product) => {
                    let has_images = !upload_files.is_empty();
                    let has_url = !img.is_empty();
                    
                    if has_images || has_url {
                        let url_opt = if has_url { Some(img) } else { None };
                        match upload_product_images(product.id, upload_files, url_opt).await {
                            Ok(_) => {
                                set_success_msg.set(Some("¡Producto editado y nuevas imágenes subidas correctamente!".to_string()));
                                // Reload product to update image list
                                product_resource.refetch();
                            }
                            Err(e) => {
                                set_error_msg.set(Some(format!("Producto editado, pero falló la subida de imágenes: {}", e)));
                            }
                        }
                    } else {
                        set_success_msg.set(Some("¡Producto editado correctamente!".to_string()));
                    }

                    if error_msg.get().is_none() {
                        let navigate = use_navigate();
                        navigate(&format!("/products/{}", pid), Default::default());
                    }
                }
                Err(e) => {
                    set_error_msg.set(Some(e));
                }
            }
        });
    };

    let delete_image = move |img_id: i64| {
        let pid = product_id();
        spawn_local(async move {
            match delete_product_image(pid, img_id).await {
                Ok(_) => {
                    set_success_msg.set(Some("Imagen eliminada correctamente".to_string()));
                    product_resource.refetch();
                }
                Err(e) => {
                    set_error_msg.set(Some(format!("Error al eliminar imagen: {}", e)));
                }
            }
        });
    };

    let on_delete_product = move |_| {
        if let Some(window) = web_sys::window() {
            if window.confirm_with_message("¿Estás seguro de que quieres borrar este producto? Esta acción no se puede deshacer.").unwrap_or(false) {
                let pid = product_id();
                let mut user_id = 0;
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(id_str)) = storage.get_item("user_id") {
                        user_id = id_str.parse::<i64>().unwrap_or(0);
                    }
                }
                
                spawn_local(async move {
                    match crate::api::products_api::delete_product(pid, user_id).await {
                        Ok(_) => {
                            if let Some(w) = web_sys::window() {
                                let _ = w.alert_with_message("Producto borrado exitosamente.");
                            }
                            let navigate = use_navigate();
                            navigate("/", Default::default());
                        }
                        Err(e) => {
                            set_error_msg.set(Some(format!("Error al borrar producto: {}", e)));
                        }
                    }
                });
            }
        }
    };

    view! {
        <div style="max-width: 600px; margin: 0 auto; background: white; padding: 20px; border-radius: 8px; box-shadow: 0 1px 3px rgba(0,0,0,0.1);">
            <h2>"Editar Producto"</h2>
            
            <Suspense fallback=move || view! { <Loading/> }>
                {move || {
                    product_resource.get().map(|res| match res {
                        Ok(_) => {
                            view! {
                                <div>
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
                                                        categories_resource.get().map(|cat_res| match cat_res {
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
                                        
                                        <div class="form-group">
                                            <label>"Descripción *"</label>
                                            <textarea prop:value=description on:input=move |ev| set_description.set(event_target_value(&ev)) required></textarea>
                                        </div>

                                        <div class="form-group" style="border: 1px solid var(--border-color); padding: 15px; border-radius: 6px; background-color: #f9fafb;">
                                            <label style="color: var(--primary-color); font-weight: bold;">"Imágenes Actuales"</label>
                                            {move || {
                                                let images = existing_images.get();
                                                if images.is_empty() {
                                                    view! { <p>"No hay imágenes actuales."</p> }.into_view()
                                                } else {
                                                    view! {
                                                        <div style="display: flex; gap: 10px; flex-wrap: wrap; margin-top: 10px;">
                                                            {images.into_iter().map(|img| {
                                                                let t_src = if img.image_url.starts_with("http") {
                                                                    img.image_url.clone()
                                                                } else {
                                                                    format!("http://127.0.0.1:3000/{}", img.image_url)
                                                                };
                                                                let img_id = img.id;
                                                                view! {
                                                                    <div style="position: relative;">
                                                                        <img src=t_src style="width: 100px; height: 100px; object-fit: cover; border-radius: 4px;"/>
                                                                        <button type="button" on:click=move |_| delete_image(img_id) style="position: absolute; top: 5px; right: 5px; background: red; color: white; border: none; border-radius: 50%; width: 24px; height: 24px; cursor: pointer;">"X"</button>
                                                                    </div>
                                                                }
                                                            }).collect_view()}
                                                        </div>
                                                    }.into_view()
                                                }
                                            }}
                                        </div>

                                        <div class="form-group" style="border: 1px solid var(--border-color); padding: 15px; border-radius: 6px; background-color: #f9fafb; margin-top: 10px;">
                                            <label style="color: var(--primary-color); font-weight: bold;">"Añadir Nuevas Imágenes"</label>
                                            <p style="font-size: 0.85rem; color: #6b7280; margin-top: 0; margin-bottom: 10px;">"Si añades fotos, se adjuntarán a las actuales."</p>
                                            <input type="file" _ref=secondary_file_input_ref accept="image/jpeg,image/png" multiple/>
                                            
                                            <div style="margin-top: 10px;">
                                                <label style="font-size: 0.9rem;">"URL de la Imagen (externa, opcional)"</label>
                                                <input type="url" prop:value=image_url on:input=move |ev| set_image_url.set(event_target_value(&ev)) placeholder="https://..."/>
                                            </div>
                                        </div>
                                        
                                        <button type="submit" class="btn" style="width: 100%; font-size: 1.1rem; padding: 12px; margin-top: 15px;">"Guardar Cambios"</button>
                                        <div style="text-align: center; margin-top: 15px; display: flex; justify-content: space-between; align-items: center;">
                                            <button type="button" on:click=on_delete_product class="btn-danger" style="background: #ef4444; color: white; padding: 8px 16px; border: none; border-radius: 4px; cursor: pointer;">"Borrar Producto"</button>
                                            <a href=format!("/products/{}", product_id()) style="color: #6b7280; text-decoration: none;">"Cancelar"</a>
                                        </div>
                                    </form>
                                </div>
                            }.into_view()
                        }
                        Err(e) => view! { <ErrorMessage message=e/> }.into_view()
                    })
                }}
            </Suspense>
        </div>
    }
}
