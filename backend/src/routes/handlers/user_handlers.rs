use crate::error;
use crate::services::user_services;
use crate::utils::{api_response::ApiResponse, app_state, jwt::Claims};
use actix_web::{get, post, web};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, Set};
use serde::{Deserialize, Serialize};

#[get("/login/verify")]
pub async fn login_verify(
    app_state: web::Data<app_state::AppState>,
    claim_data: Claims,
) -> Result<ApiResponse, ApiResponse> {
    //Verify token user
    entity::user::Entity::find_by_id(claim_data.id)
        .one(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?
        .ok_or(ApiResponse::new(404, "User not Found".to_string()))?;

    let token = user_services::get_or_create_token(claim_data, app_state)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;
    let string_response =
        serde_json::to_string(&token).map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(ApiResponse::new(200, string_response.to_owned()))
}
#[derive(Serialize, Deserialize)]
struct UpdateUser {
    name: String,
}
#[post("update")]
pub async fn update_user(
    app_state: web::Data<app_state::AppState>,
    claim_data: Claims,
    user_data: web::Json<UpdateUser>,
) -> Result<ApiResponse, ApiResponse> {
    let mut user_model = entity::user::Entity::find_by_id(claim_data.id)
        .one(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?
        .ok_or(ApiResponse::new(404, "User Not found".to_owned()))?
        .into_active_model();

    user_model.name = Set(user_data.name.clone());
    user_model
        .update(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(ApiResponse::new(200, format!("Updatd successfully")))
}
