use actix_web::{web, HttpResponse};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use mongodb::bson::doc;
use shared::enums::UserRole;
use shared::models::{AuthResponse, LoginRequest, RegisterRequest, User, UserPublic};

use crate::auth::create_token;
use crate::errors::AppError;
use crate::state::AppState;

pub async fn register(
    body: web::Json<RegisterRequest>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let collection = state.db.collection::<mongodb::bson::Document>("users");

    // Check if email already registered
    let existing = collection.find_one(doc! { "email": &body.email }).await?;

    if existing.is_some() {
        return Err(AppError::BadRequest("Email is already in use".to_string()));
    }

    let hashed_pw =
        hash(&body.password, DEFAULT_COST).map_err(|e| AppError::InternalError(e.to_string()))?;

    let new_user = User {
        id: uuid::Uuid::new_v4().to_string(),
        email: body.email.clone(),
        username: body.username.clone(),
        password_hash: hashed_pw,
        role: UserRole::User,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let doc = mongodb::bson::to_document(&new_user)?;
    collection.insert_one(doc).await?;

    let token = create_token(
        &new_user.id,
        "user",
        &state.config.jwt_secret,
        state.config.jwt_expiration_hours,
    )
    .map_err(|e| AppError::InternalError(e.to_string()))?;

    let public_user = UserPublic {
        id: new_user.id,
        email: new_user.email,
        username: new_user.username,
        role: new_user.role,
    };

    Ok(HttpResponse::Created().json(AuthResponse {
        token,
        user: public_user,
    }))
}

pub async fn login(
    body: web::Json<LoginRequest>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let collection = state.db.collection::<User>("users");

    let user = collection
        .find_one(doc! { "email": &body.email })
        .await?
        .ok_ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

    let is_valid = verify(&body.password, &user.password_hash)
        .map_err(|e| AppError::InternalError(e.to_string()))?;

    if !is_valid {
        return Err(AppError::Unauthorized("Invalid credentials".to_string()));
    }

    let role_str = match user.role {
        UserRole::Admin => "admin",
        UserRole::User => "user",
    };

    let token = create_token(
        &user.id,
        role_str,
        &state.config.jwt_secret,
        state.config.jwt_expiration_hours,
    )
    .map_err(|e| AppError::InternalError(e.to_string()))?;

    let public_user = UserPublic {
        id: user.id,
        email: user.email,
        username: user.username,
        role: user.role,
    };

    Ok(HttpResponse::Ok().json(AuthResponse {
        token,
        user: public_user,
    }))
}

// Simple extension trait for Option because we're not inside the standard rust std result context directly
trait OptionExt<T> {
    fn ok_ok_or_else<F, E>(self, f: F) -> Result<T, E>
    where
        F: FnOnce() -> E;
}

impl<T> OptionExt<T> for Option<T> {
    fn ok_ok_or_else<F, E>(self, f: F) -> Result<T, E>
    where
        F: FnOnce() -> E,
    {
        match self {
            Some(v) => Ok(v),
            None => Err(f()),
        }
    }
}
