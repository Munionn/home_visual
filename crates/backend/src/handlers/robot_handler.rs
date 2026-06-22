use actix_web::{web, HttpResponse};
use crate::errors::AppError;
use crate::state::AppState;
use crate::auth::AuthenticatedUser;
use shared::models::CreateRobotTaskRequest;

pub async fn create_task(
    _body: web::Json<CreateRobotTaskRequest>,
    _state: web::Data<AppState>,
    _auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    // TODO: Implement create robot cleaning task
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_task_status(
    _path: web::Path<String>,
    _state: web::Data<AppState>,
    _auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    // TODO: Implement get robot task status
    Ok(HttpResponse::Ok().finish())
}

pub async fn list_tasks(
    _state: web::Data<AppState>,
    _auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    // TODO: Implement list robot tasks
    Ok(HttpResponse::Ok().finish())
}

pub async fn cancel_task(
    _path: web::Path<String>,
    _state: web::Data<AppState>,
    _auth: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    // TODO: Implement cancel robot task
    Ok(HttpResponse::Ok().finish())
}
