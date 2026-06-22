use actix_web::{dev::Payload, FromRequest, HttpRequest};
use chrono::{Duration, Utc};
use futures_util::future::{ready, Ready};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

pub fn create_token(user_id: &str, role: &str, secret: &str, expiration_hours: i64) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(expiration_hours))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_owned(),
        role: role.to_owned(),
        exp: expiration as usize,
    };

    let header = Header::default();
    encode(&header, &claims, &EncodingKey::from_secret(secret.as_bytes()))
}

pub fn validate_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let mut validation = Validation::default();
    validation.validate_exp = true;
    
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;
    
    Ok(token_data.claims)
}

pub struct AuthenticatedUser {
    pub user_id: String,
    pub role: String,
}

impl FromRequest for AuthenticatedUser {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        // Extract Authorization header
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..];
                    
                    // Access AppState for the JWT Secret
                    if let Some(state) = req.app_data::<actix_web::web::Data<crate::state::AppState>>() {
                        match validate_token(token, &state.config.jwt_secret) {
                            Ok(claims) => {
                                return ready(Ok(AuthenticatedUser {
                                    user_id: claims.sub,
                                    role: claims.role,
                                }));
                            }
                            Err(_) => {
                                return ready(Err(actix_web::error::ErrorUnauthorized("Invalid Token")));
                            }
                        }
                    }
                }
            }
        }
        
        ready(Err(actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header")))
    }
}
