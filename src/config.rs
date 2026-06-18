// src/config.rs
use serde::Deserialize;
use std::sync::OnceLock;

/// Application configuration structure (Replaces your Go config.yml & Viper loader)
#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    #[serde(rename = "server_port", default = "default_port")]
    pub server_port: u16,
    
    #[serde(rename = "database_url")]
    pub database_url: String,
}

fn default_port() -> u16 {
    8080
}

// Thread-safe global read-only storage cache for configuration properties
static CONFIG: OnceLock<AppConfig> = OnceLock::new();

impl AppConfig {
    /// Loads properties from environment variables on startup.
    /// Panics directly if required parameters (like DATABASE_URL) are missing or invalid.
    pub fn load_from_env() -> &'static Self {
        CONFIG.get_or_init(|| {
            // Automatically find and parse the local .env file if it exists
            let _ = dotenvy::dotenv();

            // Feed raw environment strings directly into our Serde struct properties
            envy::from_env::<AppConfig>().unwrap_or_else(|err| {
                panic!("CRITICAL CONFIG FAILURE: Failed to load application setup. Error: {err}");
            })
        })
    }
}

