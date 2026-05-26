use sqlx::SqlitePool;
use crate::models::User;
use crate::errors::AppError;

pub async fn create_user(
    pool: &SqlitePool,
    name: &str,
    email: &str,
    password_hash: &str,
    phone: Option<&str>,
    location: Option<&str>,
) -> Result<User, AppError> {
    let query = r#"
        INSERT INTO users (name, email, password_hash, phone, location)
        VALUES (?, ?, ?, ?, ?)
        RETURNING id, name, email, password_hash, phone, location, avatar_url, created_at, updated_at
    "#;

    let user = sqlx::query_as::<_, User>(query)
        .bind(name)
        .bind(email)
        .bind(password_hash)
        .bind(phone)
        .bind(location)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            // Manejo de error de base de datos
            if let sqlx::Error::Database(db_err) = &e {
                if db_err.is_unique_violation() {
                    return AppError::ValidationError("El email ya está registrado".to_string());
                }
            }
            AppError::DatabaseError(e)
        })?;

    Ok(user)
}

pub async fn find_by_email(pool: &SqlitePool, email: &str) -> Result<Option<User>, AppError> {
    let query = "SELECT * FROM users WHERE email = ?";
    let user = sqlx::query_as::<_, User>(query)
        .bind(email)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e))?;

    Ok(user)
}

pub async fn find_by_id(pool: &SqlitePool, id: i64) -> Result<Option<User>, AppError> {
    let query = "SELECT * FROM users WHERE id = ?";
    let user = sqlx::query_as::<_, User>(query)
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e))?;

    Ok(user)
}

pub async fn update_profile(
    pool: &SqlitePool,
    id: i64,
    phone: Option<&str>,
    location: Option<&str>,
) -> Result<User, AppError> {
    let query = r#"
        UPDATE users 
        SET phone = ?, location = ?, updated_at = CURRENT_TIMESTAMP 
        WHERE id = ?
        RETURNING id, name, email, password_hash, phone, location, avatar_url, created_at, updated_at
    "#;

    let user = sqlx::query_as::<_, User>(query)
        .bind(phone)
        .bind(location)
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e))?
        .ok_or_else(|| AppError::NotFound(format!("Usuario {} no encontrado", id)))?;

    Ok(user)
}
