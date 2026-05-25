// The User response will just use the User model directly as it has #[serde(skip_serializing)] on password_hash.
// But we might need specific updates later.
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    // For now, this is a placeholder if we need it. 
    // We'll primarily return the `models::User` directly.
}
