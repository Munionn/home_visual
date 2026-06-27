use crate::enums::AutomationTriggerType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AutomationTrigger {
    pub trigger_type: AutomationTriggerType,
    pub device_id: Option<String>,
    pub schedule_cron: Option<String>,
    pub condition: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AutomationAction {
    pub device_id: String,
    pub command: String,
    pub payload: serde_json::Value,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Automation {
    pub id: String,
    pub home_id: String,
    pub name: String,
    pub enabled: bool,
    pub triggers: Vec<AutomationTrigger>,
    pub actions: Vec<AutomationAction>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateAutomationRequest {
    pub home_id: String,
    pub name: String,
    pub triggers: Vec<AutomationTrigger>,
    pub actions: Vec<AutomationAction>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateAutomationRequest {
    pub home_id: Option<String>,
    pub name: Option<String>,
    pub enabled: Option<bool>,
    pub triggers: Option<Vec<AutomationTrigger>>,
    pub actions: Option<Vec<AutomationAction>>,
}
