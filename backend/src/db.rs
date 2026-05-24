use sqlx::{sqlite::{SqliteConnectOptions, SqlitePoolOptions}, SqlitePool};
use std::str::FromStr;
use tokio::fs;

pub async fn init_db(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    // Configuramos opciones para que cree el archivo si no existe
    let options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    // Verificamos si la base de datos está vacía (comprobando si existe la tabla users)
    let table_exists: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='users'")
        .fetch_one(&pool)
        .await?;

    if table_exists.0 == 0 {
        tracing::info!("Base de datos vacía. Inicializando schema y seeds...");
        
        // Ejecutamos schema.sql
        let schema = fs::read_to_string("../database/schema.sql")
            .await
            .expect("Debería existir el archivo schema.sql");
        
        for statement in schema.split(";") {
            let stmt = statement.trim();
            if !stmt.is_empty() {
                sqlx::query(stmt).execute(&pool).await?;
            }
        }
        
        // Ejecutamos seeds.sql
        let seeds = fs::read_to_string("../database/seeds.sql")
            .await
            .expect("Debería existir el archivo seeds.sql");
        
        for statement in seeds.split(";") {
            let stmt = statement.trim();
            if !stmt.is_empty() {
                sqlx::query(stmt).execute(&pool).await?;
            }
        }
        tracing::info!("Base de datos inicializada correctamente.");
    }

    Ok(pool)
}
