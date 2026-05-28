use sqlx::SqlitePool;
use crate::dto::message_dto::{CreateMessageDto, MessageResponseDto};
use crate::errors::AppError;

pub struct MessageRepository;

impl MessageRepository {
    pub async fn create_message(pool: &SqlitePool, dto: &CreateMessageDto) -> Result<MessageResponseDto, AppError> {
        let content = dto.content.trim();
        if content.is_empty() {
            return Err(AppError::ValidationError("El mensaje no puede estar vacío".into()));
        }
        if content.len() > 1000 {
            return Err(AppError::ValidationError("El mensaje excede el límite de 1000 caracteres".into()));
        }
        if dto.sender_id == dto.receiver_id {
            return Err(AppError::ValidationError("No puedes enviarte mensajes a ti mismo".into()));
        }

        // Verificar que producto existe
        let product_exists: (i64,) = sqlx::query_as("SELECT count(id) FROM products WHERE id = ?")
            .bind(dto.product_id)
            .fetch_one(pool)
            .await
            .map_err(|e| AppError::DatabaseError(e))?;
            
        if product_exists.0 == 0 {
            return Err(AppError::ValidationError("El producto no existe".into()));
        }

        // Insertar el mensaje
        let result = sqlx::query(
            "INSERT INTO messages (product_id, sender_id, receiver_id, content) VALUES (?, ?, ?, ?)"
        )
        .bind(dto.product_id)
        .bind(dto.sender_id)
        .bind(dto.receiver_id)
        .bind(content)
        .execute(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e))?;

        let id = result.last_insert_rowid();

        Self::get_message_by_id(pool, id).await
    }

    pub async fn get_message_by_id(pool: &SqlitePool, id: i64) -> Result<MessageResponseDto, AppError> {
        let row = sqlx::query_as::<_, (i64, i64, i64, i64, String, String, i64)>(
            "SELECT id, product_id, sender_id, receiver_id, content, created_at, is_read FROM messages WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e))?;

        match row {
            Some((id, product_id, sender_id, receiver_id, content, created_at, is_read)) => {
                Ok(MessageResponseDto {
                    id, product_id, sender_id, receiver_id, content, created_at, is_read: is_read != 0
                })
            },
            None => Err(AppError::NotFound("Mensaje no encontrado".into())),
        }
    }

    pub async fn get_received_messages(pool: &SqlitePool, user_id: i64) -> Result<Vec<MessageResponseDto>, AppError> {
        let rows = sqlx::query_as::<_, (i64, i64, i64, i64, String, String, i64)>(
            "SELECT id, product_id, sender_id, receiver_id, content, created_at, is_read FROM messages WHERE receiver_id = ? ORDER BY created_at DESC"
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e))?;

        let messages = rows.into_iter().map(|(id, product_id, sender_id, receiver_id, content, created_at, is_read)| {
            MessageResponseDto {
                id, product_id, sender_id, receiver_id, content, created_at, is_read: is_read != 0
            }
        }).collect();

        Ok(messages)
    }

    pub async fn get_sent_messages(pool: &SqlitePool, user_id: i64) -> Result<Vec<MessageResponseDto>, AppError> {
        let rows = sqlx::query_as::<_, (i64, i64, i64, i64, String, String, i64)>(
            "SELECT id, product_id, sender_id, receiver_id, content, created_at, is_read FROM messages WHERE sender_id = ? ORDER BY created_at DESC"
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e))?;

        let messages = rows.into_iter().map(|(id, product_id, sender_id, receiver_id, content, created_at, is_read)| {
            MessageResponseDto {
                id, product_id, sender_id, receiver_id, content, created_at, is_read: is_read != 0
            }
        }).collect();

        Ok(messages)
    }

    pub async fn get_conversation(pool: &SqlitePool, product_id: i64, user_id: i64, other_user_id: i64) -> Result<Vec<MessageResponseDto>, AppError> {
        let rows = sqlx::query_as::<_, (i64, i64, i64, i64, String, String, i64)>(
            "SELECT id, product_id, sender_id, receiver_id, content, created_at, is_read 
             FROM messages 
             WHERE product_id = ? AND ((sender_id = ? AND receiver_id = ?) OR (sender_id = ? AND receiver_id = ?))
             ORDER BY created_at ASC"
        )
        .bind(product_id)
        .bind(user_id)
        .bind(other_user_id)
        .bind(other_user_id)
        .bind(user_id)
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e))?;

        let messages = rows.into_iter().map(|(id, product_id, sender_id, receiver_id, content, created_at, is_read)| {
            MessageResponseDto {
                id, product_id, sender_id, receiver_id, content, created_at, is_read: is_read != 0
            }
        }).collect();

        Ok(messages)
    }

    pub async fn mark_as_read(pool: &SqlitePool, message_id: i64, user_id: i64) -> Result<(), AppError> {
        let result = sqlx::query(
            "UPDATE messages SET is_read = 1 WHERE id = ? AND receiver_id = ?"
        )
        .bind(message_id)
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Mensaje no encontrado o no eres el receptor".into()));
        }

        Ok(())
    }

    pub async fn get_unread_count(pool: &SqlitePool, user_id: i64) -> Result<i64, AppError> {
        let count: (i64,) = sqlx::query_as(
            "SELECT count(*) FROM messages WHERE receiver_id = ? AND is_read = 0"
        )
        .bind(user_id)
        .fetch_one(pool)
        .await
        .unwrap_or((0,));
        
        Ok(count.0)
    }

    pub async fn block_user(pool: &SqlitePool, blocker_id: i64, blocked_id: i64) -> Result<(), AppError> {
        sqlx::query(
            "INSERT OR IGNORE INTO blocks (blocker_id, blocked_id) VALUES (?, ?)"
        )
        .bind(blocker_id)
        .bind(blocked_id)
        .execute(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e))?;
        Ok(())
    }

    pub async fn unblock_user(pool: &SqlitePool, blocker_id: i64, blocked_id: i64) -> Result<(), AppError> {
        sqlx::query(
            "DELETE FROM blocks WHERE blocker_id = ? AND blocked_id = ?"
        )
        .bind(blocker_id)
        .bind(blocked_id)
        .execute(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e))?;
        Ok(())
    }

    pub async fn get_blocks_for_user(pool: &SqlitePool, user_id: i64) -> Result<Vec<crate::dto::message_dto::BlockDto>, AppError> {
        let rows = sqlx::query_as::<_, (i64, i64)>(
            "SELECT blocker_id, blocked_id FROM blocks WHERE blocker_id = ? OR blocked_id = ?"
        )
        .bind(user_id)
        .bind(user_id)
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e))?;

        let blocks = rows.into_iter().map(|(blocker_id, blocked_id)| {
            crate::dto::message_dto::BlockDto { blocker_id, blocked_id }
        }).collect();
        Ok(blocks)
    }

    pub async fn delete_conversation(pool: &SqlitePool, user_id: i64, product_id: i64, other_user_id: i64) -> Result<(), AppError> {
        sqlx::query(
            "INSERT OR REPLACE INTO deleted_conversations (user_id, product_id, other_user_id, deleted_at) VALUES (?, ?, ?, CURRENT_TIMESTAMP)"
        )
        .bind(user_id)
        .bind(product_id)
        .bind(other_user_id)
        .execute(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e))?;
        Ok(())
    }

    pub async fn get_deleted_conversations_for_user(pool: &SqlitePool, user_id: i64) -> Result<Vec<crate::dto::message_dto::DeletedConversationDto>, AppError> {
        let rows = sqlx::query_as::<_, (i64, i64, i64)>(
            "SELECT user_id, product_id, other_user_id FROM deleted_conversations WHERE user_id = ? OR other_user_id = ?"
        )
        .bind(user_id)
        .bind(user_id)
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e))?;

        let deleted = rows.into_iter().map(|(u_id, product_id, other_user_id)| {
            crate::dto::message_dto::DeletedConversationDto { user_id: u_id, product_id, other_user_id }
        }).collect();
        Ok(deleted)
    }
}
