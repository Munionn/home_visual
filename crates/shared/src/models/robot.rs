use crate::enums::{RobotStatus, RobotTaskType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RobotTask {
    pub id: String,
    pub robot_device_id: String,
    pub task_type: RobotTaskType,
    pub target_room_id: Option<String>,
    pub status: RobotStatus,
    pub progress_percent: f32,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateRobotTaskRequest {
    pub robot_device_id: String,
    pub task_type: RobotTaskType,
    pub target_room_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RobotPosition {
    pub x: f64,
    pub y: f64,
    pub heading: f64,
}
