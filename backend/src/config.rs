use dotenvy::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
}

impl Config {
    pub fn init() -> Self {
        // Ignoramos si no hay archivo .env, útil para CI o si las variables están seteadas a nivel OS.
        let _ = dotenv();

        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "sqlite://../database/nebripop.db".to_string());
        
        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .expect("PORT debe ser un número válido");

        Self {
            database_url,
            port,
        }
    }
}
