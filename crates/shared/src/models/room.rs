use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlacedItem {
    pub id: String,
    pub name: String,
    pub item_type: String,
    pub position: Point2D,
    pub rotation: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Room {
    pub id: String,
    pub home_id: String,
    pub name: String,
    pub width: f64,
    pub height: f64,
    pub position: Point2D,
    pub polygon_points: Vec<Point2D>,
    pub placed_items: Vec<PlacedItem>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateRoomRequest {
    pub home_id: String,
    pub name: String,
    pub width: f64,
    pub height: f64,
    pub position: Point2D,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateRoomRequest {
    pub name: Option<String>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub position: Option<Point2D>,
    pub polygon_points: Option<Vec<Point2D>>,
    pub placed_items: Option<Vec<PlacedItem>>,
}
