use crate::auth::AuthenticatedUser;
use crate::errors::AppError;
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use chrono::Utc;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use shared::{
    models::{CreateDeviceRequest, DeviceCommand, UpdateDeviceRequest},
    Device,
};

pub async fn create_device(
    body: web::Json<CreateDeviceRequest>,
    state: web::Data<AppState>,
    _auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let collection = state.db.collection::<Device>("devices");

    let new_device = Device {
        id: ObjectId::new().to_hex(),
        home_id: body.home_id.clone(),
        room_id: body.room_id.clone(),
        name: body.name.clone(),
        device_type: body.device_type.clone(),
        state: shared::enums::DeviceState::Unknown,
        power_state: shared::enums::DevicePowerState::Off,
        mqtt_topic: body.mqtt_topic.clone(),
        properties: serde_json::Value::Object(serde_json::Map::new()),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    collection
        .insert_one(&new_device)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "message": "Device created successfully",
        "id": new_device.id
    })))
}

pub async fn get_device(
    path: web::Path<String>,
    state: web::Data<AppState>,
    _auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let collection = state.db.collection::<Device>("devices");
    let filter = doc! { "_id": path.into_inner() };
    let device = collection
        .find_one(filter)
        .await?
        .ok_or(AppError::NotFound("Device not found".to_string()))?;
    Ok(HttpResponse::Ok().json(device))
}

pub async fn list_devices(
    state: web::Data<AppState>,
    path: web::Path<String>,
    _auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let collection = state.db.collection::<Device>("devices");
    let mut cursor = collection
        .find(doc! {"_id":path.into_inner()})
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let mut devices = Vec::new();
    while cursor
        .advance()
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
    {
        devices.push(
            cursor
                .deserialize_current()
                .map_err(|e| AppError::DatabaseError(e.to_string()))?,
        );
    }

    Ok(HttpResponse::Ok().json(devices))
}

pub async fn update_device(
    path: web::Path<String>,
    body: web::Json<UpdateDeviceRequest>,
    state: web::Data<AppState>,
    _auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    // TODO: Implement update device
    let collection = state.db.collection::<Device>("devices");
    let device_id = path.into_inner();
    let mut update_doc = doc! {"$set": {"update_at": Utc::now()}};
    let set_doc = update_doc
        .get_document_mut("$set")
        .map_err(|e| AppError::InternalError(e.to_string()))?;

    if let Some(name) = body.name.clone() {
        set_doc.insert("name", name);
    }
    if let Some(room_id) = body.room_id.clone() {
        set_doc.insert("room_id", room_id);
    }
    if let Some(mqtt_topic) = body.mqtt_topic.clone() {
        set_doc.insert("mqtt_topic", mqtt_topic);
    }
    if let Some(state) = body.state.clone() {
        set_doc.insert(
            "state",
            mongodb::bson::to_bson(&state).map_err(|e| AppError::InternalError(e.to_string()))?,
        );
    }
    if let Some(power_state) = body.power_state.clone() {
        set_doc.insert(
            "power_state",
            mongodb::bson::to_bson(&power_state)
                .map_err(|e| AppError::InternalError(e.to_string()))?,
        );
    }
    if let Some(properties) = body.properties.clone() {
        set_doc.insert(
            "properties",
            mongodb::bson::to_bson(&properties)
                .map_err(|e| AppError::InternalError(e.to_string()))?,
        );
    }

    let result = collection
        .find_one_and_update(doc! {"_id": device_id}, update_doc)
        .await?;
    if result.is_some() {
        Ok(HttpResponse::Ok().json("Succesfully update device "))
    } else {
        Err(AppError::NotFound(
            "Device not found or unable to update it".to_string(),
        ))
    }
}

pub async fn delete_device(
    path: web::Path<String>,
    state: web::Data<AppState>,
    _auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let device_id = path.into_inner();
    let collection = state.db.collection::<Device>("devices");
    let filter = doc! {"_id": &device_id};

    let deleted_device = collection
        .find_one_and_delete(filter)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    match deleted_device {
        Some(_) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "Device deleted successfully",
            "id": device_id
        }))),
        None => Err(AppError::NotFound(format!(
            "Device {} not found",
            device_id
        ))),
    }
}

pub async fn send_command(
    path: web::Path<String>,
    body: web::Json<DeviceCommand>,
    state: web::Data<AppState>,
    _auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let device_id = path.into_inner();
    let collection = state.db.collection::<Device>("devices");

    let device = collection
        .find_one(doc! {"_id": &device_id})
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFound(format!("Device {} not found", device_id)))?;

    let payload =
        serde_json::to_string(&body.payload).map_err(|e| AppError::BadRequest(e.to_string()))?;

    state
        .mqtt
        .publish(&device.mqtt_topic, &payload)
        .await
        .map_err(|e| AppError::MqttError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Command sent successfully",
        "topic": device.mqtt_topic
    })))
}
