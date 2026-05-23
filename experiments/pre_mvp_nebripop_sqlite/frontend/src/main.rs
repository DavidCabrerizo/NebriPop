use leptos::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Product {
    id: i64,
    title: String,
    description: String,
    price: f64,
    category: String,
    condition: String,
    location: String,
    contact: String,
    created_at: String,
}

#[derive(Serialize)]
struct NewProduct {
    title: String,
    description: String,
    price: f64,
    category: String,
    condition: String,
    location: String,
    contact: String,
}

#[component]
fn App() -> impl IntoView {
    let (title, set_title) = create_signal(String::new());
    let (description, set_description) = create_signal(String::new());
    let (price_str, set_price_str) = create_signal(String::new());
    let (category, set_category) = create_signal(String::new());
    let (condition, set_condition) = create_signal(String::new());
    let (location, set_location) = create_signal(String::new());
    let (contact, set_contact) = create_signal(String::new());

    let (error_msg, set_error_msg) = create_signal(String::new());
    let (success_msg, set_success_msg) = create_signal(String::new());

    let fetch_products = create_resource(
        || (),
        |_| async move {
            let client = Client::new();
            match client.get("http://127.0.0.1:3000/products").send().await {
                Ok(resp) => {
                    if let Ok(data) = resp.json::<Vec<Product>>().await {
                        data
                    } else {
                        vec![]
                    }
                },
                Err(_) => vec![],
            }
        },
    );

    let submit_form = move |_| {
        set_error_msg.set(String::new());
        set_success_msg.set(String::new());

        let t = title.get();
        let d = description.get();
        let p_str = price_str.get();
        let ctg = category.get();
        let cnd = condition.get();
        let loc = location.get();
        let cnt = contact.get();

        if t.trim().is_empty() || d.trim().is_empty() || p_str.trim().is_empty() || ctg.trim().is_empty() || cnd.trim().is_empty() || loc.trim().is_empty() || cnt.trim().is_empty() {
            set_error_msg.set("Todos los campos son obligatorios".to_string());
            return;
        }

        let p: f64 = match p_str.parse() {
            Ok(val) if val > 0.0 => val,
            _ => {
                set_error_msg.set("El precio debe ser un número mayor que 0".to_string());
                return;
            }
        };

        let payload = NewProduct {
            title: t,
            description: d,
            price: p,
            category: ctg,
            condition: cnd,
            location: loc,
            contact: cnt,
        };

        spawn_local(async move {
            let client = Client::new();
            match client.post("http://127.0.0.1:3000/products")
                .json(&payload)
                .send()
                .await
            {
                Ok(resp) => {
                    if resp.status().is_success() {
                        set_success_msg.set("Producto publicado con éxito!".to_string());
                        
                        // Limpiar formulario
                        set_title.set(String::new());
                        set_description.set(String::new());
                        set_price_str.set(String::new());
                        set_category.set(String::new());
                        set_condition.set(String::new());
                        set_location.set(String::new());
                        set_contact.set(String::new());

                        // Actualizar lista
                        fetch_products.refetch();
                    } else {
                        set_error_msg.set("Error desde el servidor al guardar".to_string());
                    }
                },
                Err(_) => {
                    set_error_msg.set("Error de conexión con el backend".to_string());
                }
            }
        });
    };

    view! {
        <div style="font-family: sans-serif; max-width: 800px; margin: 2rem auto; padding: 2rem;">
            <h1 style="text-align: center;">"NebriPop Pre-MVP"</h1>
            
            <div style="border: 1px solid #ddd; padding: 1.5rem; border-radius: 8px; margin-bottom: 2rem;">
                <h2>"Formulario para publicar producto"</h2>
                
                <div style="display: flex; flex-direction: column; gap: 1rem;">
                    <input type="text" placeholder="Título" on:input=move |ev| set_title.set(event_target_value(&ev)) prop:value=title style="padding: 0.5rem;" />
                    <textarea placeholder="Descripción" on:input=move |ev| set_description.set(event_target_value(&ev)) prop:value=description style="padding: 0.5rem;" />
                    <input type="number" placeholder="Precio" on:input=move |ev| set_price_str.set(event_target_value(&ev)) prop:value=price_str style="padding: 0.5rem;" />
                    <input type="text" placeholder="Categoría" on:input=move |ev| set_category.set(event_target_value(&ev)) prop:value=category style="padding: 0.5rem;" />
                    <input type="text" placeholder="Estado (ej. Usado)" on:input=move |ev| set_condition.set(event_target_value(&ev)) prop:value=condition style="padding: 0.5rem;" />
                    <input type="text" placeholder="Ubicación" on:input=move |ev| set_location.set(event_target_value(&ev)) prop:value=location style="padding: 0.5rem;" />
                    <input type="text" placeholder="Contacto (usuario)" on:input=move |ev| set_contact.set(event_target_value(&ev)) prop:value=contact style="padding: 0.5rem;" />
                    
                    <button on:click=submit_form style="padding: 0.8rem; background-color: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer; font-weight: bold;">
                        "Publicar producto"
                    </button>
                </div>
                
                <p style="color: red;">{error_msg}</p>
                <p style="color: green;">{success_msg}</p>
            </div>

            <div>
                <h2>"Listado de productos publicados"</h2>
                <Suspense fallback=move || view! { <p>"Cargando productos..."</p> }>
                    {move || {
                        fetch_products.get().map(|prods| {
                            if prods.is_empty() {
                                view! { <p>"No hay productos publicados todavía."</p> }.into_view()
                            } else {
                                view! {
                                    <div style="display: grid; gap: 1rem;">
                                        {prods.into_iter().map(|p| {
                                            view! {
                                                <div style="border: 1px solid #ccc; padding: 1rem; border-radius: 8px; background-color: #f9f9f9;">
                                                    <h3 style="margin-top: 0;">{p.title}</h3>
                                                    <p style="margin: 0.2rem 0;"><strong>"Descripción: "</strong> {p.description}</p>
                                                    <p style="margin: 0.2rem 0;"><strong>"Precio: "</strong> {p.price} " €"</p>
                                                    <p style="margin: 0.2rem 0;"><strong>"Categoría: "</strong> {p.category}</p>
                                                    <p style="margin: 0.2rem 0;"><strong>"Estado: "</strong> {p.condition}</p>
                                                    <p style="margin: 0.2rem 0;"><strong>"Ubicación: "</strong> {p.location}</p>
                                                    <p style="margin: 0.2rem 0;"><strong>"Contacto: "</strong> {p.contact}</p>
                                                    <p style="font-size: 0.8rem; color: #666; margin-top: 0.5rem;">"Publicado el: " {p.created_at}</p>
                                                </div>
                                            }
                                        }).collect_view()}
                                    </div>
                                }.into_view()
                            }
                        })
                    }}
                </Suspense>
            </div>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App/> })
}
