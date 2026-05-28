use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageResponseDto {
    pub id: i64,
    pub product_id: i64,
    pub sender_id: i64,
    pub receiver_id: i64,
    pub content: String,
    pub created_at: String,
    pub is_read: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateMessageDto {
    pub product_id: i64,
    pub sender_id: i64,
    pub receiver_id: i64,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMessageReadDto {
    pub user_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct ConversationQueryDto {
    pub user_id: i64,
    pub other_user_id: i64,
}
