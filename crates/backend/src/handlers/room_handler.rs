use actix_web::{web, HttpResponse};
use chrono::Utc;
use mongodb::bson::doc;
use shared::models::{CreateRoomRequest, Room, UpdateRoomRequest};
use uuid::Uuid;

use crate::auth::AuthenticatedUser;
use crate::errors::AppError;
use crate::state::AppState;

pub async fn create_room(
    body: web::Json<CreateRoomRequest>,
    state: web::Data<AppState>,
    _auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let collection = state.db.collection::<Room>("rooms");
    
    let new_room = Room {
        id: Uuid::new_v4().to_string(),
        home_id: body.home_id.clone(),
        name: body.name.clone(),
        width: body.width,
        height: body.height,
        position: body.position,
        polygon_points: Vec::new(),
        placed_items: Vec::new(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    collection.insert_one(new_room.clone()).await?;

    // Also update the Home to include this Room
    let home_collection = state.db.collection::<mongodb::bson::Document>("homes");
    home_collection
        .update_one(
            doc! { "id": &body.home_id },
            doc! { "$push": { "room_ids": &new_room.id } },
        )
        .await?;

    Ok(HttpResponse::Created().json(new_room))
}

pub async fn list_rooms(
    query: web::Query<std::collections::HashMap<String, String>>,
    state: web::Data<AppState>,
    _auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let collection = state.db.collection::<Room>("rooms");
    
    let mut filter = doc! {};
    if let Some(home_id) = query.get("home_id") {
        filter.insert("home_id", home_id);
    }

    let mut cursor = collection.find(filter).await?;
    let mut rooms = Vec::new();
    while cursor.advance().await? {
        rooms.push(cursor.deserialize_current()?);
    }

    Ok(HttpResponse::Ok().json(rooms))
}

pub async fn get_room(
    path: web::Path<String>,
    state: web::Data<AppState>,
    _auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let room_id = path.into_inner();
    let collection = state.db.collection::<Room>("rooms");
    
    let room = collection
        .find_one(doc! { "id": &room_id })
        .await?;

    match room {
        Some(r) => Ok(HttpResponse::Ok().json(r)),
        None => Err(AppError::NotFound("Room not found".to_string())),
    }
}

pub async fn update_room(
    path: web::Path<String>,
    body: web::Json<UpdateRoomRequest>,
    state: web::Data<AppState>,
    _auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let room_id = path.into_inner();
    let collection = state.db.collection::<Room>("rooms");
    
    let mut update_doc = doc! {};
    if let Some(ref name) = body.name {
        update_doc.insert("name", name);
    }
    if let Some(width) = body.width {
        update_doc.insert("width", width);
    }
    if let Some(height) = body.height {
        update_doc.insert("height", height);
    }
    if let Some(ref pos) = body.position {
        update_doc.insert("position", doc! { "x": pos.x, "y": pos.y });
    }
    if let Some(ref points) = body.polygon_points {
        let serialized_points = mongodb::bson::to_bson(points)?;
        update_doc.insert("polygon_points", serialized_points);
    }
    if let Some(ref items) = body.placed_items {
        let serialized_items = mongodb::bson::to_bson(items)?;
        update_doc.insert("placed_items", serialized_items);
    }
    update_doc.insert("updated_at", Utc::now().to_rfc3339());

    let result = collection
        .update_one(
            doc! { "id": &room_id },
            doc! { "$set": update_doc },
        )
        .await?;

    if result.matched_count == 0 {
        return Err(AppError::NotFound("Room not found".to_string()));
    }

    let updated = collection
        .find_one(doc! { "id": &room_id })
        .await?
        .ok_or_else(|| AppError::NotFound("Room not found after update".to_string()))?;

    Ok(HttpResponse::Ok().json(updated))
}

pub async fn delete_room(
    path: web::Path<String>,
    state: web::Data<AppState>,
    _auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let room_id = path.into_inner();
    let collection = state.db.collection::<Room>("rooms");
    
    // Find room to get home_id first
    if let Some(room) = collection.find_one(doc! { "id": &room_id }).await? {
        // Delete room
        collection.delete_one(doc! { "id": &room_id }).await?;
        
        // Remove room reference from home
        let home_collection = state.db.collection::<mongodb::bson::Document>("homes");
        home_collection
            .update_one(
                doc! { "id": &room.home_id },
                doc! { "$pull": { "room_ids": &room_id } },
            )
            .await?;
        
        return Ok(HttpResponse::NoContent().finish());
    }

    Err(AppError::NotFound("Room not found".to_string()))
}
