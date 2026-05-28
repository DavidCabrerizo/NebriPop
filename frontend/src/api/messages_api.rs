use crate::{
    api::client::{get_url, handle_error, handle_network_error},
    models::message::{CreateMessageDto, Message},
};
use reqwest::Client;
use serde_json::json;

pub async fn get_received_messages(user_id: i64) -> Result<Vec<Message>, String> {
    let url = get_url(&format!("/users/{}/messages", user_id));
    let client = Client::new();
    
    match client.get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                let messages = response.json::<Vec<Message>>().await.map_err(|e| e.to_string())?;
                Ok(messages)
            } else {
                Err(handle_error(response).await)
            }
        }
        Err(e) => Err(handle_network_error(e)),
    }
}

pub async fn get_sent_messages(user_id: i64) -> Result<Vec<Message>, String> {
    let url = get_url(&format!("/users/{}/messages/sent", user_id));
    let client = Client::new();
    
    match client.get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                let messages = response.json::<Vec<Message>>().await.map_err(|e| e.to_string())?;
                Ok(messages)
            } else {
                Err(handle_error(response).await)
            }
        }
        Err(e) => Err(handle_network_error(e)),
    }
}

pub async fn get_conversation(product_id: i64, user_id: i64, other_user_id: i64) -> Result<Vec<Message>, String> {
    let url = get_url(&format!("/products/{}/messages?user_id={}&other_user_id={}", product_id, user_id, other_user_id));
    let client = Client::new();
    
    match client.get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                let messages = response.json::<Vec<Message>>().await.map_err(|e| e.to_string())?;
                Ok(messages)
            } else {
                Err(handle_error(response).await)
            }
        }
        Err(e) => Err(handle_network_error(e)),
    }
}

pub async fn create_message(dto: CreateMessageDto) -> Result<Message, String> {
    let url = get_url("/messages");
    let client = Client::new();
    
    match client.post(&url).json(&dto).send().await {
        Ok(response) => {
            if response.status().is_success() {
                let msg = response.json::<Message>().await.map_err(|e| e.to_string())?;
                Ok(msg)
            } else {
                Err(handle_error(response).await)
            }
        }
        Err(e) => Err(handle_network_error(e)),
    }
}

pub async fn mark_as_read(message_id: i64, user_id: i64) -> Result<(), String> {
    let url = get_url(&format!("/messages/{}/read", message_id));
    let client = Client::new();
    let payload = json!({ "user_id": user_id });
    
    match client.patch(&url).json(&payload).send().await {
        Ok(res) if res.status().is_success() => Ok(()),
        Ok(res) => Err(handle_error(res).await),
        Err(e) => Err(handle_network_error(e)),
    }
}

pub async fn get_unread_count(user_id: i64) -> Result<i64, String> {
    let url = get_url(&format!("/users/{}/messages/unread", user_id));
    let client = Client::new();
    
    match client.get(&url).send().await {
        Ok(res) if res.status().is_success() => {
            let data: serde_json::Value = res.json().await.unwrap_or(json!({ "unread_count": 0 }));
            Ok(data["unread_count"].as_i64().unwrap_or(0))
        }
        Ok(res) => Err(handle_error(res).await),
        Err(e) => Err(handle_network_error(e)),
    }
}

pub async fn block_user(blocker_id: i64, blocked_id: i64) -> Result<(), String> {
    let url = get_url("/users/block");
    let client = Client::new();
    let payload = json!({ "blocker_id": blocker_id, "blocked_id": blocked_id });
    
    match client.post(&url).json(&payload).send().await {
        Ok(res) if res.status().is_success() => Ok(()),
        Ok(res) => Err(handle_error(res).await),
        Err(e) => Err(handle_network_error(e)),
    }
}

pub async fn unblock_user(blocker_id: i64, blocked_id: i64) -> Result<(), String> {
    let url = get_url("/users/unblock");
    let client = Client::new();
    let payload = json!({ "blocker_id": blocker_id, "blocked_id": blocked_id });
    
    match client.post(&url).json(&payload).send().await {
        Ok(res) if res.status().is_success() => Ok(()),
        Ok(res) => Err(handle_error(res).await),
        Err(e) => Err(handle_network_error(e)),
    }
}

pub async fn delete_conversation(user_id: i64, product_id: i64, other_user_id: i64) -> Result<(), String> {
    let url = get_url("/messages/conversation/delete");
    let client = Client::new();
    let payload = json!({ "user_id": user_id, "product_id": product_id, "other_user_id": other_user_id });
    
    match client.post(&url).json(&payload).send().await {
        Ok(res) if res.status().is_success() => Ok(()),
        Ok(res) => Err(handle_error(res).await),
        Err(e) => Err(handle_network_error(e)),
    }
}

pub async fn get_blocks(user_id: i64) -> Result<Vec<crate::models::message::BlockDto>, String> {
    let url = get_url(&format!("/users/{}/blocks", user_id));
    let client = Client::new();
    
    match client.get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                let blocks = response.json::<Vec<crate::models::message::BlockDto>>().await.map_err(|e| e.to_string())?;
                Ok(blocks)
            } else {
                Err(handle_error(response).await)
            }
        }
        Err(e) => Err(handle_network_error(e)),
    }
}

pub async fn get_deleted_conversations(user_id: i64) -> Result<Vec<crate::models::message::DeletedConversationDto>, String> {
    let url = get_url(&format!("/users/{}/messages/deleted", user_id));
    let client = Client::new();
    
    match client.get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                let deleted = response.json::<Vec<crate::models::message::DeletedConversationDto>>().await.map_err(|e| e.to_string())?;
                Ok(deleted)
            } else {
                Err(handle_error(response).await)
            }
        }
        Err(e) => Err(handle_network_error(e)),
    }
}
