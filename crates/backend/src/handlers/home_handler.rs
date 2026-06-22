use actix_web::{web, HttpResponse};
use chrono::Utc;
use mongodb::bson::doc;
use shared::models::{CreateHomeRequest, Home, UpdateHomeRequest};
use uuid::Uuid;

use crate::auth::AuthenticatedUser;
use crate::errors::AppError;
use crate::state::AppState;

pub async fn create_home(
    body: web::Json<CreateHomeRequest>,
    state: web::Data<AppState>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let collection = state.db.collection::<Home>("homes");
    
    let new_home = Home {
        id: Uuid::new_v4().to_string(),
        name: body.name.clone(),
        owner_id: auth.user_id,
        room_ids: Vec::new(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    collection.insert_one(new_home.clone()).await?;

    Ok(HttpResponse::Created().json(new_home))
}

pub async fn list_homes(
    state: web::Data<AppState>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let collection = state.db.collection::<Home>("homes");
    let mut cursor = collection.find(doc! { "owner_id": &auth.user_id }).await?;
    
    let mut homes = Vec::new();
    while cursor.advance().await? {
        homes.push(cursor.deserialize_current()?);
    }

    Ok(HttpResponse::Ok().json(homes))
}

pub async fn get_home(
    path: web::Path<String>,
    state: web::Data<AppState>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let home_id = path.into_inner();
    let collection = state.db.collection::<Home>("homes");
    
    let home = collection
        .find_one(doc! { "id": &home_id, "owner_id": &auth.user_id })
        .await?;

    match home {
        Some(h) => Ok(HttpResponse::Ok().json(h)),
        None => Err(AppError::NotFound("Home not found".to_string())),
    }
}

pub async fn update_home(
    path: web::Path<String>,
    body: web::Json<UpdateHomeRequest>,
    state: web::Data<AppState>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let home_id = path.into_inner();
    let collection = state.db.collection::<Home>("homes");
    
    let mut update_doc = doc! {};
    if let Some(ref name) = body.name {
        update_doc.insert("name", name);
    }
    update_doc.insert("updated_at", Utc::now().to_rfc3339());

    let result = collection
        .update_one(
            doc! { "id": &home_id, "owner_id": &auth.user_id },
            doc! { "$set": update_doc },
        )
        .await?;

    if result.matched_count == 0 {
        return Err(AppError::NotFound("Home not found".to_string()));
    }

    let updated = collection
        .find_one(doc! { "id": &home_id })
        .await?
        .ok_or_else(|| AppError::NotFound("Home not found after update".to_string()))?;

    Ok(HttpResponse::Ok().json(updated))
}

pub async fn delete_home(
    path: web::Path<String>,
    state: web::Data<AppState>,
    auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let home_id = path.into_inner();
    let collection = state.db.collection::<Home>("homes");
    
    let result = collection
        .delete_one(doc! { "id": &home_id, "owner_id": &auth.user_id })
        .await?;

    if result.deleted_count == 0 {
        return Err(AppError::NotFound("Home not found".to_string()));
    }

    Ok(HttpResponse::NoContent().finish())
}
