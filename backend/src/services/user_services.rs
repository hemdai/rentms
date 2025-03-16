use crate::models::token_model::CreateTokenModel;
use crate::models::token_model::TokenResponse;
use crate::utils::app_state;
use crate::utils::jwt;
use crate::utils::jwt::Claims;
use actix_web::web;
use chrono::Utc;
use entity::sea_orm_active_enums::TokenTypeEnum;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

pub async fn get_or_create_token(
    claims: Claims,
    app_state: web::Data<app_state::AppState>,
) -> Result<TokenResponse, String> {
    let token_record = entity::token::Entity::find()
        .filter(entity::token::Column::UserId.eq(claims.id))
        .one(&app_state.db)
        .await
        .map_err(|err| err.to_string())
        .unwrap();

    match token_record {
        Some(token) => Ok(TokenResponse {
            id: token.id,
            created_at: token.created_at,
            key: token.key,
            token_type: token.token_type,
            user_id: token.user_id,
        }),
        None => {
            insert_token_to_db(
                CreateTokenModel {
                    key: jwt::encode_jwt(claims.email, claims.id).unwrap(),
                    user_id: claims.id,
                    token_type: TokenTypeEnum::AccessToken.to_owned(),
                },
                app_state,
            )
            .await
        }
    }
}

pub async fn insert_token_to_db(
    insert_token: CreateTokenModel,
    app_state: web::Data<app_state::AppState>,
) -> Result<TokenResponse, String> {
    let token_record = entity::token::ActiveModel {
        key: Set(insert_token.key.to_string()),
        user_id: Set(insert_token.user_id),
        created_at: Set(Utc::now().naive_local()),
        token_type: Set(insert_token.token_type),
        ..Default::default()
    }
    .insert(&app_state.db)
    .await
    .map_err(|error| format!("Couldnot insert Token to DB {}", error.to_string()))
    .unwrap();

    let token_response = TokenResponse {
        id: token_record.id,
        created_at: token_record.created_at,
        key: token_record.key,
        token_type: token_record.token_type,
        user_id: token_record.user_id,
    };
    Ok(token_response)
}
