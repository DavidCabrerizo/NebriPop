use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub phone: Option<String>,
    pub location: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    // For now, this is a placeholder if we need it. 
    // We'll primarily return the `models::User` directly.
}
