use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SceneAction {
    pub device_id: String,
    pub command: String,
    pub payload: serde_json::Value,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Scene {
    pub id: String,
    pub home_id: String,
    pub name: String,
    pub description: Option<String>,
    pub actions: Vec<SceneAction>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateSceneRequest {
    pub home_id: String,
    pub name: String,
    pub description: Option<String>,
    pub actions: Vec<SceneAction>,
}
