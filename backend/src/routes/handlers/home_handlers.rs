use crate::utils::{
    api_response::{self, ApiResponse},
    app_state::AppState,
    constants::MEDIA_DIRECTORY,
};
use actix_web::{get, web, HttpResponse, Responder};
use sea_orm::{ConnectionTrait, Statement};
use serde::Deserialize;

#[get("/")]
pub async fn hello() -> impl Responder {
    api_response::ApiResponse::new(200, "hello Welcome to rentms !".to_string())
}

#[get("/users")]
pub async fn get_users() -> impl Responder {
    HttpResponse::Ok().json(vec!["user1", "user2", "user3"])
}

#[get("/rents")]
pub async fn get_rents(app_state: web::Data<AppState>) -> Result<ApiResponse, ApiResponse> {
    let _res = app_state
        .db
        .query_all(Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT * FROM user; ",
        ))
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(api_response::ApiResponse::new(200, "Test".to_string()))
}

#[get("/great/{name}")]
pub async fn great(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}! timro hal khavar k cha bhai", name))
}
use std::path::PathBuf;
#[derive(Deserialize)]
struct ImageQuery {
    w: Option<u32>, // Width (optional)
    q: Option<u8>,  // Quality (optional)
}

#[get("/static/images/{filename:.*}")]
async fn serve_image(path: web::Path<String>, _query: web::Query<ImageQuery>) -> impl Responder {
    let image_path: PathBuf =
        format!("{}/{}", MEDIA_DIRECTORY.to_string(), path.into_inner()).into();

    println!("Image path: {}", image_path.display());

    if image_path.exists() {
        HttpResponse::Ok()
            .content_type("image/jpeg") // Adjust based on file type
            .body(std::fs::read(image_path).unwrap())
    } else {
        HttpResponse::NotFound().body("Image not found")
    }
}
