use axum::{extract::State, response::Json};
use sqlx::SqlitePool;
use bcrypt::{hash, verify, DEFAULT_COST};

use crate::dto::auth_dto::{LoginRequest, RegisterRequest, AuthResponse};
use crate::errors::AppError;
use crate::repositories::user_repository;

pub async fn register(
    State(pool): State<SqlitePool>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    // Basic validation
    if payload.name.trim().is_empty() {
        return Err(AppError::ValidationError("Name is required".into()));
    }
    if payload.email.trim().is_empty() || !payload.email.contains('@') {
        return Err(AppError::ValidationError("Valid email is required".into()));
    }
    if payload.password.trim().len() < 6 {
        return Err(AppError::ValidationError("Password must be at least 6 characters".into()));
    }

    let hashed_password = hash(&payload.password, DEFAULT_COST)
        .map_err(|_| AppError::InternalServerError)?;

    let user = user_repository::create_user(
        &pool,
        &payload.name,
        &payload.email,
        &hashed_password,
        payload.phone.as_deref(),
        payload.location.as_deref(),
    ).await?;

    Ok(Json(AuthResponse {
        user,
        message: "Registro exitoso".into(),
    }))
}

pub async fn login(
    State(pool): State<SqlitePool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let user = user_repository::find_by_email(&pool, &payload.email)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Email o contraseña incorrectos".into()))?;

    let is_valid = verify(&payload.password, &user.password_hash)
        .unwrap_or(false);

    if !is_valid {
        return Err(AppError::Unauthorized("Email o contraseña incorrectos".into()));
    }

    Ok(Json(AuthResponse {
        user,
        message: "Login correcto".into(),
    }))
}
