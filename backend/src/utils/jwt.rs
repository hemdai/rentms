use std::future;

use super::api_response::ApiResponse;
use super::app_state;
use crate::utils::constants;
use actix_web::{web, FromRequest, HttpMessage};
use chrono::DateTime;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Header, Validation};
use jsonwebtoken::{DecodingKey, EncodingKey};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub email: String,
    pub exp: usize,
    pub iat: usize,
    pub id: i32,
}

impl FromRequest for Claims {
    type Error = actix_web::Error;
    type Future = future::Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> std::future::Ready<Result<Claims, actix_web::Error>> {
        match req.extensions().get::<Claims>() {
            Some(claim) => future::ready(Ok(claim.clone())),
            None => future::ready(Err(actix_web::error::ErrorBadRequest("Bad Claims"))),
        }
    }
}

pub fn encode_jwt(email: String, id: i32) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let expire = Duration::days(30);
    let claims = Claims {
        exp: (now + expire).timestamp() as usize,
        iat: now.timestamp() as usize,
        id,
        email,
    };

    let secret = constants::SECRET_KEY.clone();
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap();
    Ok(token)
}

pub fn decode_jwt(token: String) -> Result<Claims, String> {
    let secret = constants::SECRET_KEY.clone();
    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    ) {
        Ok(token_data) => Ok(token_data.claims),
        Err(err) => match err.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                Err("Token has expired".to_string())
            }
            _ => Err(format!("Invalid token: {}", err)),
        },
    }
}

pub async fn check_token_in_db(
    token: String,
    app_state: web::Data<app_state::AppState>,
) -> Result<CustomClaims, ApiResponse> {
    let result = entity::token::Entity::find()
        .filter(entity::token::Column::Key.eq(&token))
        .find_also_related(entity::user::Entity)
        .one(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    match result {
        Some((token, user)) => {
            // Token found
            let user_name = user
                .map(|u| u.name)
                .unwrap_or_else(|| "Unknown".to_string());
            // Use token and user_name as needed
            let created_at_utc = DateTime::<Utc>::from_naive_utc_and_offset(token.created_at, Utc);
            let expiray_date = created_at_utc + Duration::days(30);
            if expiray_date >= Utc::now() {
                let claim = CustomClaims {
                    sub: token.id.to_string(),
                    name: user_name.clone(),
                    role: "user".to_string(),
                    exp: expiray_date.clone(),
                    is_valid: true,
                };
                print!("Token is valid from db the user is {}", user_name);
                return Ok(claim);
            }
            return Ok(CustomClaims::default());
        }
        None => return Ok(CustomClaims::default()),
    }
}

#[derive(Serialize, Deserialize)]
pub struct CustomClaims {
    pub sub: String,
    pub name: String,
    pub role: String,
    pub exp: DateTime<Utc>,
    pub is_valid: bool,
}

impl Default for CustomClaims {
    fn default() -> Self {
        CustomClaims {
            sub: "".to_string(),
            name: "".to_string(),
            role: "".to_string(),
            exp: Utc::now(),
            is_valid: false,
        }
    }
}
