use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum DeviceType {
    Light,
    CoffeeMachine,
    RobotCleaner,
    Switch,
    Sensor,
    Thermostat,
    Camera,
    Lock,
    Speaker,
    Other,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum DeviceState {
    Online,
    Offline,
    Error,
    Unknown,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum DevicePowerState {
    On,
    Off,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum RobotStatus {
    Idle,
    Cleaning,
    Returning,
    Charging,
    Error,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum RobotTaskType {
    CleanFullFlat,
    CleanRoom,
    CleanZone,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum UserRole {
    Admin,
    User,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum AutomationTriggerType {
    DeviceState,
    Schedule,
    Sunrise,
    Sunset,
    Manual,
}
