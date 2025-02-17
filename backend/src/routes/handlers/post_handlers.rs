use std::path::PathBuf;

use crate::utils;
use crate::utils::{api_response::ApiResponse, app_state, jwt::Claims};
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_multipart::form::MultipartForm;
use actix_web::{get, post, web};
use chrono::NaiveDateTime;
use chrono::Utc;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::Set;
use sea_orm::{ActiveModelTrait, TransactionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(MultipartForm)]
pub struct CretePostModel {
    title: Text<String>,
    text: Text<String>,
    file: TempFile,
}

#[derive(Serialize, Deserialize)]
pub struct PostModel {
    pub id: i32,
    pub title: String,
    pub text: String,
    pub uuid: Uuid,
    pub image: String,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
}

#[post("create")]
pub async fn create_post(
    app_state: web::Data<app_state::AppState>,
    claim: Claims,
    post_model: MultipartForm<CretePostModel>,
) -> Result<ApiResponse, ApiResponse> {
    let check_name = post_model
        .file
        .file_name
        .clone()
        .unwrap_or("null".to_string());
    let max_file_size = (utils::constants::MAX_FILE_SIZE).clone();

    match &check_name[check_name.len() - 4..] {
        ".png" | ".jpg" => {}
        _ => return Err(ApiResponse::new(401, " Invalid File type".to_string())),
    };

    match post_model.file.size {
        0 => return Err(ApiResponse::new(401, "No File".to_owned())),
        length if length > max_file_size as usize => {
            return Err(ApiResponse::new(
                401,
                "File size not alloud more than 10M".to_owned(),
            ))
        }
        _ => {}
    };
    let txn = app_state
        .db
        .begin()
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    let post_entity = entity::post::ActiveModel {
        title: Set(post_model.title.clone()),
        text: Set(post_model.text.clone()),
        uuid: Set(Uuid::new_v4()),
        user_id: Set(claim.id),
        created_at: Set(Utc::now().naive_local()),
        image: Set("No Image".to_string()),
        ..Default::default()
    };

    let mut created_entity = post_entity
        .save(&txn)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    let temp_file_path = post_model.file.file.path();
    let file_name = post_model
        .file
        .file_name
        .as_ref()
        .map(|m| m.as_ref())
        .unwrap_or("null");
    let time_stamp: i64 = Utc::now().timestamp();

    let mut file_path = PathBuf::from("./public");

    let new_file_name = format!("{}-{}", time_stamp, file_name);
    file_path.push(&new_file_name);

    match std::fs::copy(temp_file_path, file_path) {
        Ok(_) => {
            created_entity.image = Set(new_file_name);
            created_entity
                .save(&txn)
                .await
                .map_err(|err| ApiResponse::new(500, err.to_string()))?;
            txn.commit()
                .await
                .map_err(|err| ApiResponse::new(500, err.to_string()))?;

            std::fs::remove_file(temp_file_path).unwrap_or_default();

            Ok(ApiResponse::new(200, "Post Created".to_owned()))
        }
        Err(_) => {
            txn.rollback()
                .await
                .map_err(|err| ApiResponse::new(500, err.to_string()))?;
            Err(ApiResponse::new(500, "Internal Server Error".to_owned()))
        }
    }
}

#[get("my-posts")]
pub async fn my_posts(
    claim: Claims,
    app_state: web::Data<app_state::AppState>,
) -> Result<ApiResponse, ApiResponse> {
    let posts: Vec<PostModel> = entity::post::Entity::find()
        .filter(entity::post::Column::UserId.eq(claim.id))
        .all(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?
        .into_iter()
        .map(|post| PostModel {
            id: post.id,
            title: post.title,
            text: post.text,
            uuid: post.uuid,
            image: post.image,
            user_id: post.user_id,
            created_at: post.created_at,
        })
        .collect();

    let res_str =
        serde_json::to_string(&posts).map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(ApiResponse::new(200, res_str.to_owned()))
}

#[get("all-post")]
pub async fn get_all_post(
    app_state: web::Data<app_state::AppState>,
) -> Result<ApiResponse, ApiResponse> {
    let all_post: Vec<PostModel> = entity::post::Entity::find()
        .all(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?
        .into_iter()
        .map(|post| PostModel {
            id: post.id,
            title: post.title,
            text: post.text,
            uuid: post.uuid,
            image: post.image,
            user_id: post.user_id,
            created_at: post.created_at,
        })
        .collect();
    let string_result =
        serde_json::to_string(&all_post).map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(ApiResponse::new(200, string_result.to_owned()))
}

#[get("{post_uuid}")]
pub async fn get_post_by_id(
    app_state: web::Data<app_state::AppState>,
    post_uuid: web::Path<Uuid>,
) -> Result<ApiResponse, ApiResponse> {
    let post_result: PostModel = entity::post::Entity::find()
        .filter(entity::post::Column::Uuid.eq(post_uuid.clone()))
        .one(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?
        .map(|post| PostModel {
            id: post.id,
            title: post.title,
            text: post.text,
            uuid: post.uuid,
            image: post.image,
            user_id: post.user_id,
            created_at: post.created_at,
        })
        .ok_or(ApiResponse::new(404, "Post Not Found".to_string()))?;

    let result_str = serde_json::to_string(&post_result)
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;
    Ok(ApiResponse::new(200, result_str.to_owned()))
}
