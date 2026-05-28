use axum::{
    routing::{get, post, patch},
    Router,
};
use sqlx::SqlitePool;

use crate::handlers::messages_handler::{
    create_message,
    get_user_received_messages,
    get_user_sent_messages,
    get_conversation,
    mark_message_as_read,
    get_unread_messages_count,
    block_user_handler,
    get_blocks_handler,
    delete_conversation_handler,
    get_deleted_conversations_handler,
    unblock_user_handler,
};

pub fn routes(pool: SqlitePool) -> Router {
    Router::new()
        .route("/messages", post(create_message))
        .route("/users/:id/messages", get(get_user_received_messages))
        .route("/users/:id/messages/sent", get(get_user_sent_messages))
        .route("/users/:id/messages/unread", get(get_unread_messages_count))
        .route("/products/:product_id/messages", get(get_conversation))
        .route("/messages/:id/read", patch(mark_message_as_read))
        .route("/messages/conversation/delete", post(delete_conversation_handler))
        .route("/users/block", post(block_user_handler))
        .route("/users/unblock", post(unblock_user_handler))
        .route("/users/:id/blocks", get(get_blocks_handler))
        .route("/users/:id/messages/deleted", get(get_deleted_conversations_handler))
        .with_state(pool)
}
