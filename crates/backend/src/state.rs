use crate::config::AppConfig;
use crate::mqtt::MqttClient;
use mongodb::Database;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub mqtt: MqttClient,
    pub config: AppConfig,
}
