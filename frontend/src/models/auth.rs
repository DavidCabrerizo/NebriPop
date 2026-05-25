use serde::{Deserialize, Serialize};
use crate::models::user::User;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub user: User,
    pub message: String,
}
