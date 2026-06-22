use std::env;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub mongodb_uri: String,
    pub mongodb_database: String,
    pub mqtt_broker_host: String,
    pub mqtt_broker_port: u16,
    pub mqtt_client_id: String,
    pub jwt_secret: String,
    pub jwt_expiration_hours: i64,
    pub server_host: String,
    pub server_port: u16,
    pub ha_url: Option<String>,
    pub ha_token: Option<String>,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, anyhow::Error> {
        // Load .env file if present
        let _ = dotenvy::dotenv();

        Ok(Self {
            mongodb_uri: env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string()),
            mongodb_database: env::var("MONGODB_DATABASE").unwrap_or_else(|_| "home_visual".to_string()),
            mqtt_broker_host: env::var("MQTT_BROKER_HOST").unwrap_or_else(|_| "localhost".to_string()),
            mqtt_broker_port: env::var("MQTT_BROKER_PORT")
                .unwrap_or_else(|_| "1883".to_string())
                .parse()?,
            mqtt_client_id: env::var("MQTT_CLIENT_ID").unwrap_or_else(|_| "home_visual_backend".to_string()),
            jwt_secret: env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret_key_change_me".to_string()),
            jwt_expiration_hours: env::var("JWT_EXPIRATION_HOURS")
                .unwrap_or_else(|_| "24".to_string())
                .parse()?,
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()?,
            ha_url: env::var("HA_URL").ok(),
            ha_token: env::var("HA_TOKEN").ok(),
        })
    }
}
