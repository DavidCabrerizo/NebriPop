use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: i64,
    pub product_id: i64,
    pub sender_id: i64,
    pub receiver_id: i64,
    pub content: String,
    pub created_at: String,
    pub is_read: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMessageDto {
    pub product_id: i64,
    pub sender_id: i64,
    pub receiver_id: i64,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationItem {
    pub product_id: i64,
    pub other_user_id: i64,
    pub product_title: String,
    pub last_message: String,
    pub unread_count: usize,
    pub updated_at: String,
    pub is_blocked_by_other: bool,
    pub is_deleted_by_other: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockDto {
    pub blocker_id: i64,
    pub blocked_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletedConversationDto {
    pub user_id: i64,
    pub product_id: i64,
    pub other_user_id: i64,
}
