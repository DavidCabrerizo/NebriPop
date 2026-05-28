use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;

use crate::{
    dto::message_dto::{CreateMessageDto, ConversationQueryDto, UpdateMessageReadDto, BlockUserRequest, DeleteConversationRequest},
    errors::AppError,
    repositories::message_repository::MessageRepository,
};

pub async fn create_message(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateMessageDto>,
) -> Result<(StatusCode, Json<crate::dto::message_dto::MessageResponseDto>), AppError> {
    let msg = MessageRepository::create_message(&pool, &payload).await?;
    Ok((StatusCode::CREATED, Json(msg)))
}

pub async fn get_user_received_messages(
    State(pool): State<SqlitePool>,
    Path(user_id): Path<i64>,
) -> Result<Json<Vec<crate::dto::message_dto::MessageResponseDto>>, AppError> {
    let messages = MessageRepository::get_received_messages(&pool, user_id).await?;
    Ok(Json(messages))
}

pub async fn get_user_sent_messages(
    State(pool): State<SqlitePool>,
    Path(user_id): Path<i64>,
) -> Result<Json<Vec<crate::dto::message_dto::MessageResponseDto>>, AppError> {
    let messages = MessageRepository::get_sent_messages(&pool, user_id).await?;
    Ok(Json(messages))
}

pub async fn get_conversation(
    State(pool): State<SqlitePool>,
    Path(product_id): Path<i64>,
    Query(query): Query<ConversationQueryDto>,
) -> Result<Json<Vec<crate::dto::message_dto::MessageResponseDto>>, AppError> {
    let messages = MessageRepository::get_conversation(&pool, product_id, query.user_id, query.other_user_id).await?;
    Ok(Json(messages))
}

pub async fn mark_message_as_read(
    State(pool): State<SqlitePool>,
    Path(message_id): Path<i64>,
    Json(payload): Json<UpdateMessageReadDto>,
) -> Result<Json<serde_json::Value>, AppError> {
    MessageRepository::mark_as_read(&pool, message_id, payload.user_id).await?;
    Ok(Json(serde_json::json!({
        "message": "Mensaje marcado como leído",
        "id": message_id
    })))
}

pub async fn get_unread_messages_count(
    State(pool): State<SqlitePool>,
    Path(user_id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    let count: i64 = MessageRepository::get_unread_count(&pool, user_id).await?;
    Ok(Json(serde_json::json!({
        "unread_count": count
    })))
}

pub async fn block_user_handler(
    State(pool): State<SqlitePool>,
    Json(payload): Json<BlockUserRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    MessageRepository::block_user(&pool, payload.blocker_id, payload.blocked_id).await?;
    Ok(Json(serde_json::json!({ "message": "Usuario bloqueado con éxito" })))
}

pub async fn unblock_user_handler(
    State(pool): State<SqlitePool>,
    Json(payload): Json<BlockUserRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    MessageRepository::unblock_user(&pool, payload.blocker_id, payload.blocked_id).await?;
    Ok(Json(serde_json::json!({ "message": "Usuario desbloqueado con éxito" })))
}

pub async fn get_blocks_handler(
    State(pool): State<SqlitePool>,
    Path(user_id): Path<i64>,
) -> Result<Json<Vec<crate::dto::message_dto::BlockDto>>, AppError> {
    let blocks = MessageRepository::get_blocks_for_user(&pool, user_id).await?;
    Ok(Json(blocks))
}

pub async fn delete_conversation_handler(
    State(pool): State<SqlitePool>,
    Json(payload): Json<DeleteConversationRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    MessageRepository::delete_conversation(&pool, payload.user_id, payload.product_id, payload.other_user_id).await?;
    Ok(Json(serde_json::json!({ "message": "Conversación borrada con éxito" })))
}

pub async fn get_deleted_conversations_handler(
    State(pool): State<SqlitePool>,
    Path(user_id): Path<i64>,
) -> Result<Json<Vec<crate::dto::message_dto::DeletedConversationDto>>, AppError> {
    let deleted = MessageRepository::get_deleted_conversations_for_user(&pool, user_id).await?;
    Ok(Json(deleted))
}
