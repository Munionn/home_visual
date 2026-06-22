use crate::enums::{DevicePowerState, DeviceState, DeviceType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Device {
    pub id: String,
    pub home_id: String,
    pub room_id: Option<String>,
    pub name: String,
    pub device_type: DeviceType,
    pub state: DeviceState,
    pub power_state: DevicePowerState,
    pub mqtt_topic: String,
    pub properties: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateDeviceRequest {
    pub home_id: String,
    pub room_id: Option<String>,
    pub name: String,
    pub device_type: DeviceType,
    pub mqtt_topic: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateDeviceRequest {
    pub name: Option<String>,
    pub room_id: Option<String>,
    pub mqtt_topic: Option<String>,
    pub state: Option<DeviceState>,
    pub power_state: Option<DevicePowerState>,
    pub properties: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeviceCommand {
    pub command: String,
    pub payload: serde_json::Value,
}
