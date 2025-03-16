use crate::models::token_model::CreateTokenModel;
use crate::services::user_services;
use crate::utils::api_response;
use crate::utils::api_response::ApiResponse;
use crate::utils::app_state;
use crate::utils::jwt;
use actix_web::{get, post, web, Responder};
use entity::sea_orm_active_enums::TokenTypeEnum;
use entity::user::Column;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use sha256::digest;
use std::fmt::Debug;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct RegisterModel {
    name: String,
    email: String,
    password1: String,
    password2: String,
}

#[derive(serde::Deserialize)]
struct LoginModel {
    email: String,
    password: String,
}

#[get("/auth")]
pub async fn hello_user() -> impl Responder {
    api_response::ApiResponse::new(200, "hello Welcome to hello user !".to_string())
}
#[derive(Serialize, Deserialize)]
pub struct ErrorMessage {
    message: String,
    status: i32,
}
#[post("/register")]
pub async fn register(
    app_state: web::Data<app_state::AppState>,
    register_json: web::Json<RegisterModel>,
) -> Result<ApiResponse, ApiResponse> {
    dbg!(&register_json, "debug log is here");
    if register_json.password1.clone() != register_json.password2.clone() {
        let message_obj = ErrorMessage {
            message: "Password doesn't match".to_string(),
            status: 400,
        };
        let string_message = serde_json::to_string(&message_obj)
            .map_err(|err| ApiResponse::new(500, err.to_string()))?;
        return Err(api_response::ApiResponse::new(
            400,
            string_message.to_owned(),
        ));
    }
    let user_model = entity::user::ActiveModel {
        name: Set(register_json.name.clone()),
        email: Set(register_json.email.clone()),
        password: Set(digest(register_json.password1.clone())),
        ..Default::default()
    }
    .insert(&app_state.db)
    .await
    .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    let token = jwt::encode_jwt(user_model.email, user_model.id)
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;
    let create_token_model = CreateTokenModel {
        key: token,
        user_id: user_model.id,
        token_type: TokenTypeEnum::AccessToken.to_owned(),
    };

    let token_record = user_services::insert_token_to_db(create_token_model, app_state)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    let string_record = serde_json::to_string(&token_record)
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(api_response::ApiResponse::new(200, string_record))
}

#[post("/get-user")]
pub async fn get_user(
    app_state: web::Data<app_state::AppState>,
    login_json: web::Json<LoginModel>,
) -> Result<ApiResponse, ApiResponse> {
    let user_data = entity::user::Entity::find()
        .filter(
            Condition::all()
                .add(Column::Email.eq(&login_json.email))
                .add(Column::Password.eq(digest(&login_json.password))),
        )
        .one(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?
        .ok_or(ApiResponse::new(404, "User Not Found".to_string()))?;

    let token: String = jwt::encode_jwt(user_data.email, user_data.id)
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;
    let create_token_model = CreateTokenModel {
        key: token,
        user_id: user_data.id,
        token_type: TokenTypeEnum::AccessToken.to_owned(),
    };

    let inserted_record = user_services::insert_token_to_db(create_token_model, app_state)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;
    let string_record = serde_json::to_string(&inserted_record)
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(api_response::ApiResponse::new(200, string_record))
}
