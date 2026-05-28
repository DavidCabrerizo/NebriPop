use crate::api::messages_api::{get_received_messages, get_sent_messages};
use crate::components::error_message::ErrorMessage;
use crate::components::loading::Loading;
use crate::models::message::Message;
use leptos::*;
use leptos_router::*;
use std::collections::HashMap;

#[component]
pub fn Messages() -> impl IntoView {
    let (current_user_id, set_current_user_id) = create_signal(0i64);
    
    create_effect(move |_| {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(id_str)) = storage.get_item("user_id") {
                    set_current_user_id.set(id_str.parse::<i64>().unwrap_or(0));
                }
            }
        }
    });

    let messages_resource = create_resource(move || current_user_id.get(), |user_id| async move {
        if user_id == 0 {
            return Err("Inicia sesión para ver tus mensajes".to_string());
        }
        
        let received = get_received_messages(user_id).await.unwrap_or_default();
        let sent = get_sent_messages(user_id).await.unwrap_or_default();
        
        // Agrupar por (product_id, other_user_id) para mostrar "conversaciones"
        let mut convs: HashMap<(i64, i64), Vec<Message>> = HashMap::new();
        for m in received.into_iter().chain(sent.into_iter()) {
            let other = if m.sender_id == user_id { m.receiver_id } else { m.sender_id };
            convs.entry((m.product_id, other)).or_insert_with(Vec::new).push(m);
        }
        
        let mut results = Vec::new();
        for ((product_id, other_id), mut msgs) in convs {
            msgs.sort_by(|a, b| b.created_at.cmp(&a.created_at)); // más reciente primero
            if let Some(last) = msgs.first() {
                let product_name = match crate::api::products_api::fetch_product_by_id(product_id).await {
                    Ok(p) => p.product.title,
                    Err(_) => format!("Producto #{}", product_id),
                };
                let user_name = match crate::api::users_api::get_user_profile(other_id).await {
                    Ok(u) => u.name,
                    Err(_) => format!("Usuario #{}", other_id),
                };
                
                results.push((product_id, other_id, product_name, user_name, last.content.clone(), last.created_at.clone()));
            }
        }
        results.sort_by(|a, b| b.5.cmp(&a.5));
        
        Ok(results)
    });

    view! {
        <div class="messages-page">
            <h2>"Tus Conversaciones"</h2>
            <Suspense fallback=move || view! { <Loading/> }>
                {move || {
                    if current_user_id.get() == 0 {
                        return view! { <p>"Debes iniciar sesión para ver tus mensajes."</p> }.into_view();
                    }
                    
                    messages_resource.get().map(|result| match result {
                        Ok(conversations) => {
                            if conversations.is_empty() {
                                view! { <p>"No tienes mensajes todavía."</p> }.into_view()
                            } else {
                                view! {
                                    <div style="display: flex; flex-direction: column; gap: 10px;">
                                        {conversations.into_iter().map(|(prod_id, other_id, prod_name, user_name, last_msg, date)| {
                                            let link = format!("/products/{}/conversation/{}", prod_id, other_id);
                                            view! {
                                                <div class="card" style="display: flex; justify-content: space-between; align-items: center;">
                                                    <div>
                                                        <h3 style="margin-top: 0;">{prod_name}</h3>
                                                        <p style="color: #666; margin-bottom: 5px;">"Último mensaje: " {last_msg}</p>
                                                        <small>"Usuario: " {user_name} " - " {date}</small>
                                                    </div>
                                                    <A href=link class="btn btn-secondary">"Ver Conversación"</A>
                                                </div>
                                            }
                                        }).collect_view()}
                                    </div>
                                }.into_view()
                            }
                        }
                        Err(e) => view! { <ErrorMessage message=e/> }.into_view()
                    }).unwrap_or_else(|| view! { <div></div> }.into_view())
                }}
            </Suspense>
        </div>
    }
}
