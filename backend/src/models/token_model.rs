use chrono::Utc;
use entity::sea_orm_active_enums::TokenTypeEnum;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateTokenModel {
    pub key: String,
    pub user_id: i32,
    pub token_type: TokenTypeEnum,
}

impl Default for CreateTokenModel {
    fn default() -> Self {
        CreateTokenModel {
            key: "".to_string(),
            user_id: 0,
            token_type: TokenTypeEnum::AccessToken,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TokenResponse {
    pub id: i32,
    pub key: String,
    pub created_at: DateTime,
    pub token_type: TokenTypeEnum,
    pub user_id: i32,
}
