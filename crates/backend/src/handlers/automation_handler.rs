use crate::auth::AuthenticatedUser;
use crate::errors::AppError;
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use chrono::Utc;
use mongodb::bson::{doc, oid::ObjectId};
use serde::Deserialize;
use shared::models::CreateAutomationRequest;
use shared::{Automation, UpdateAutomationRequest};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct AutomationPath {
    home_id: String,
    automation_id: String,
}

pub async fn create_automation(
    body: web::Json<CreateAutomationRequest>,
    state: web::Data<AppState>,
    _auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let collection = state.db.collection::<Automation>("automations");

    let new_automation = Automation {
        id: ObjectId::new().to_hex(),
        home_id: body.home_id.clone(),
        name: body.name.clone(),
        enabled: true,
        triggers: body.triggers.clone(),
        actions: body.actions.clone(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    collection.insert_one(new_automation.clone()).await?;

    Ok(HttpResponse::Created().json(new_automation))
}

pub async fn get_automation(
    _path: web::Path<AutomationPath>,
    _state: web::Data<AppState>,
    _auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let params = _path.into_inner();
    let collection = _state.db.collection::<Automation>("automations");
    let filter = doc! {"id": params.automation_id, "home_id": params.home_id};
    let automation = collection.find_one(filter).await?;
    Ok(HttpResponse::Ok().json(automation))
}

use futures_util::StreamExt;

pub async fn list_automations(
    path: web::Path<String>,
    state: web::Data<AppState>,
    _auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let filter = doc! {"home_id": path.into_inner()};
    let collection = state.db.collection::<Automation>("automations");

    let mut cursor = collection.find(filter).await?;
    let mut automations = Vec::new();

    while let Some(automation) = cursor.next().await {
        automations.push(automation?);
    }

    Ok(HttpResponse::Ok().json(automations))
}

pub async fn update_automation(
    _path: web::Path<String>,
    _body: web::Json<UpdateAutomationRequest>,
    _state: web::Data<AppState>,
    _auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let automation_id = _path.into_inner();
    let collection = _state.db.collection::<Automation>("automations");
    let mut update_doc = doc! { "$set": {"updated_at": Utc::now()} };

    let set_doc = update_doc
        .get_document_mut("$set")
        .map_err(|e| AppError::InternalError(e.to_string()))?;

    if let Some(home_id) = _body.home_id.clone() {
        set_doc.insert("home_id", home_id);
    }
    if let Some(name) = _body.name.clone() {
        set_doc.insert("name", name);
    }
    if let Some(enabled) = _body.enabled {
        set_doc.insert("enabled", enabled);
    }
    if let Some(triggers) = _body.triggers.clone() {
        set_doc.insert(
            "triggers",
            mongodb::bson::to_bson(&triggers)
                .map_err(|e| AppError::InternalError(e.to_string()))?,
        );
    }
    if let Some(actions) = _body.actions.clone() {
        set_doc.insert(
            "actions",
            mongodb::bson::to_bson(&actions).map_err(|e| AppError::InternalError(e.to_string()))?,
        );
    }

    let result = collection
        .find_one_and_update(doc! { "id": automation_id }, update_doc)
        .await?;

    if result.is_some() {
        Ok(HttpResponse::Ok().finish())
    } else {
        Err(AppError::NotFound("Automation not found".to_string()))
    }
}
pub async fn delete_automation(
    _path: web::Path<String>,
    _state: web::Data<AppState>,
    _auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let automation_id = Arc::new(_path.into_inner());
    let collection = _state.db.collection::<Automation>("automations");
    let ft = doc! {"id": <std::string::String as Clone>::clone(&*automation_id)};
    let deleted_automation = collection.find_one_and_delete(ft).await?;

    match deleted_automation {
        Some(_) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "Automation deleted successfully",
            "id": *automation_id
        }))),
        None => Err(AppError::NotFound(format!(
            "Automation {} not found",
            automation_id
        ))),
    }
}

pub async fn toggle_automation(
    path: web::Path<String>,
    state: web::Data<AppState>,
    _auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let automation_id = path.into_inner();
    let collection = state.db.collection::<Automation>("automations");

    let filter = doc! { "id": &automation_id };

    // Fetch the current state
    let automation = collection
        .find_one(filter.clone())
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Automation {} not found", automation_id)))?;

    // Toggle the state
    let new_enabled = !automation.enabled;
    let update = doc! {
        "$set": {
            "enabled": new_enabled,
            "updated_at": Utc::now()
        }
    };

    collection
        .find_one_and_update(filter, update)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Automation {} not found", automation_id)))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Automation toggled",
        "id": automation_id,
        "enabled": new_enabled
    })))
}
