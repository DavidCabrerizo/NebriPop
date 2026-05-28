use crate::api::messages_api::{create_message, get_conversation, mark_as_read};
use crate::api::products_api::fetch_product_by_id;
use crate::components::error_message::ErrorMessage;
use crate::components::loading::Loading;
use crate::models::message::CreateMessageDto;
use leptos::*;
use leptos_router::*;

#[derive(Params, PartialEq, Clone)]
struct ConversationParams {
    id: Option<String>,
    other_id: Option<String>,
}

#[component]
pub fn Conversation() -> impl IntoView {
    let params = use_params::<ConversationParams>();
    let (current_user_id, set_current_user_id) = create_signal(0i64);
    let (new_msg, set_new_msg) = create_signal(String::new());
    let (error, set_error) = create_signal(String::new());
    let (refresh_trigger, set_refresh_trigger) = create_signal(0);
    let (is_action_loading, set_action_loading) = create_signal(false);
    let navigate = use_navigate();
    let set_unread_count = use_context::<WriteSignal<i64>>();
    
    create_effect(move |_| {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(id_str)) = storage.get_item("user_id") {
                    set_current_user_id.set(id_str.parse::<i64>().unwrap_or(0));
                }
            }
        }
    });

    let product_id = move || {
        params.with(|p| p.as_ref().ok().and_then(|p| p.id.clone()).and_then(|id| id.parse::<i64>().ok()).unwrap_or(0))
    };

    let other_user_id = move || {
        params.with(|p| p.as_ref().ok().and_then(|p| p.other_id.clone()).and_then(|id| id.parse::<i64>().ok()).unwrap_or(0))
    };

    let data_resource = create_resource(
        move || (product_id(), current_user_id.get(), other_user_id(), refresh_trigger.get()), 
        move |(prod_id, user_id, other_id, _)| async move {
            if prod_id == 0 || user_id == 0 || other_id == 0 {
                return Err("Datos inválidos".to_string());
            }
            let product_detail = fetch_product_by_id(prod_id).await?;
            let msgs = get_conversation(prod_id, user_id, other_id).await?;
            
            let mut marked_any = false;
            // Marcar como leídos los que son para mí
            for m in &msgs {
                if !m.is_read && m.receiver_id == user_id {
                    let _ = mark_as_read(m.id, user_id).await;
                    marked_any = true;
                }
            }
            
            if marked_any {
                if let Some(set_unread) = set_unread_count {
                    if let Ok(c) = crate::api::messages_api::get_unread_count(user_id).await {
                        set_unread.set(c);
                    }
                }
            }
            
            let blocks = crate::api::messages_api::get_blocks(user_id).await.unwrap_or_default();
            let deleted = crate::api::messages_api::get_deleted_conversations(user_id).await.unwrap_or_default();
            
            let they_blocked = blocks.iter().any(|b| b.blocker_id == other_id && b.blocked_id == user_id);
            let i_blocked = blocks.iter().any(|b| b.blocker_id == user_id && b.blocked_id == other_id);
            let they_deleted = deleted.iter().any(|d| d.user_id == other_id && d.product_id == prod_id && d.other_user_id == user_id);
            let i_deleted = deleted.iter().any(|d| d.user_id == user_id && d.product_id == prod_id && d.other_user_id == other_id);
            
            Ok((product_detail.product, msgs, they_blocked, they_deleted, i_deleted, i_blocked))
        }
    );

    let do_delete = move |_| {
        if is_action_loading.get() { return; }
        let p_id = product_id();
        let u_id = current_user_id.get();
        let o_id = other_user_id();
        set_action_loading.set(true);
        spawn_local(async move {
            let _ = crate::api::messages_api::delete_conversation(u_id, p_id, o_id).await;
            if let Some(w) = web_sys::window() {
                let _ = w.location().set_href("/messages");
            }
        });
    };

    let do_block = move |_| {
        if is_action_loading.get() { return; }
        let u_id = current_user_id.get();
        let o_id = other_user_id();
        set_action_loading.set(true);
        spawn_local(async move {
            let _ = crate::api::messages_api::block_user(u_id, o_id).await;
            set_action_loading.set(false);
            set_refresh_trigger.update(|n| *n += 1);
        });
    };

    let do_unblock = move |_| {
        if is_action_loading.get() { return; }
        let u_id = current_user_id.get();
        let o_id = other_user_id();
        set_action_loading.set(true);
        spawn_local(async move {
            let _ = crate::api::messages_api::unblock_user(u_id, o_id).await;
            set_action_loading.set(false);
            set_refresh_trigger.update(|n| *n += 1);
        });
    };

    let send_message = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        set_error.set(String::new());
        let text = new_msg.get();
        if text.trim().is_empty() {
            set_error.set("El mensaje no puede estar vacío".to_string());
            return;
        }

        spawn_local(async move {
            let p_id = product_id();
            let u_id = current_user_id.get();
            let rec_id = other_user_id();

            if let Some(Ok((_prod, _msgs, _, _, _, _))) = data_resource.get() {
                let dto = CreateMessageDto {
                    product_id: p_id,
                    sender_id: u_id,
                    receiver_id: rec_id,
                    content: text,
                };

                match create_message(dto).await {
                    Ok(_) => {
                        set_new_msg.set(String::new());
                        set_refresh_trigger.update(|n| *n += 1);
                    }
                    Err(e) => set_error.set(e),
                }
            }
        });
    };

    view! {
        <div class="conversation-page">
            <div style="margin-bottom: 20px;">
                <A href="/messages" class="btn btn-secondary">"← Volver a Mensajes"</A>
            </div>
            
            <Suspense fallback=move || view! { <Loading/> }>
                {move || {
                    if current_user_id.get() == 0 {
                        return view! { <p>"Debes iniciar sesión para ver esta conversación."</p> }.into_view();
                    }
                    
                    data_resource.get().map(|result| match result {
                        Ok((prod, msgs, they_blocked, they_deleted, i_deleted, i_blocked)) => {
                            view! {
                                <div>
                                    <div class="card" style="margin-bottom: 20px; display: flex; justify-content: space-between; align-items: center;">
                                        <div style="display: flex; gap: 15px; align-items: center;">
                                            {if let Some(img) = prod.main_image_url {
                                            let src = if img.starts_with("http") { img } else { format!("http://127.0.0.1:3000/{}", img) };
                                            view! { <img src=src style="width: 80px; height: 80px; object-fit: cover; border-radius: 4px;" /> }.into_view()
                                        } else { view! { <div></div> }.into_view() }}
                                        <div>
                                            <h2 style="margin: 0;"><A href=format!("/products/{}", prod.id)>{prod.title}</A></h2>
                                            <p style="margin: 5px 0 0 0; color: var(--primary-color); font-weight: bold;">{prod.price} " €"</p>
                                        </div>
                                        </div>
                                        <div style="display: flex; gap: 10px;">
                                            <button class="btn btn-danger" style="background-color: #dc3545; color: white;" on:click=do_delete disabled=is_action_loading>"Borrar Chat"</button>
                                            {if i_blocked {
                                                view! { <button class="btn btn-secondary" style="background-color: #6c757d; color: white;" on:click=do_unblock disabled=is_action_loading>"Desbloquear Usuario"</button> }.into_view()
                                            } else {
                                                view! { <button class="btn btn-danger" style="background-color: #6c757d; color: white;" on:click=do_block disabled=is_action_loading>"Bloquear Usuario"</button> }.into_view()
                                            }}
                                        </div>
                                    </div>

                                    <div class="messages-list" style="background: #fff; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); margin-bottom: 20px; max-height: 400px; overflow-y: auto;">
                                        {if msgs.is_empty() {
                                            view! { <p style="text-align: center; color: #666;">"Aún no hay mensajes. ¡Escribe el primero!"</p> }.into_view()
                                        } else {
                                            msgs.into_iter().map(|m| {
                                                let is_mine = m.sender_id == current_user_id.get();
                                                let align = if is_mine { "flex-end" } else { "flex-start" };
                                                let bg = if is_mine { "#e3f2fd" } else { "#f5f5f5" };
                                                view! {
                                                    <div style=format!("display: flex; flex-direction: column; align-items: {}; margin-bottom: 15px;", align)>
                                                        <div style=format!("background: {}; padding: 10px 15px; border-radius: 8px; max-width: 70%;", bg)>
                                                            <p style="margin: 0;">{m.content}</p>
                                                        </div>
                                                        <small style="color: #999; margin-top: 4px; font-size: 0.75rem;">
                                                            {if is_mine && m.is_read { "Leído - " } else { "" }}
                                                            {m.created_at}
                                                        </small>
                                                    </div>
                                                }
                                            }).collect_view()
                                        }}
                                    </div>
                                    {if they_blocked {
                                        view! {
                                            <div style="background-color: #ffebee; color: #c62828; padding: 15px; border-radius: 8px; text-align: center; font-weight: bold;">
                                                "El usuario te ha bloqueado."
                                            </div>
                                        }.into_view()
                                    } else if they_deleted {
                                        view! {
                                            <div style="background-color: #fff3e0; color: #e65100; padding: 15px; border-radius: 8px; text-align: center; font-weight: bold;">
                                                "El usuario ha borrado la conversación en común."
                                            </div>
                                        }.into_view()
                                    } else if i_deleted {
                                        view! {
                                            <div style="background-color: #e8eaf6; color: #283593; padding: 15px; border-radius: 8px; text-align: center; font-weight: bold;">
                                                "Has borrado esta conversación."
                                            </div>
                                        }.into_view()
                                    } else if i_blocked {
                                        view! {
                                            <div style="background-color: #ffebee; color: #c62828; padding: 15px; border-radius: 8px; text-align: center; font-weight: bold;">
                                                "Has bloqueado a este usuario. Desbloquéalo para enviar mensajes."
                                            </div>
                                        }.into_view()
                                    } else {
                                        view! {
                                            <form on:submit=send_message style="display: flex; gap: 10px;">
                                                <input 
                                                    type="text" 
                                                    placeholder="Escribe un mensaje..." 
                                                    class="form-control" 
                                                    style="flex: 1;"
                                                    prop:value=new_msg
                                                    on:input=move |ev| set_new_msg.set(event_target_value(&ev))
                                                />
                                                <button type="submit" class="btn btn-primary">"Enviar"</button>
                                            </form>
                                        }.into_view()
                                    }}
                                    {move || {
                                        let e = error.get();
                                        if !e.is_empty() {
                                            view! { <ErrorMessage message=e /> }.into_view()
                                        } else {
                                            view! { <div></div> }.into_view()
                                        }
                                    }}
                                </div>
                            }.into_view()
                        }
                        Err(e) => view! { <ErrorMessage message=e/> }.into_view()
                    }).unwrap_or_else(|| view! { <div></div> }.into_view())
                }}
            </Suspense>
        </div>
    }
}
