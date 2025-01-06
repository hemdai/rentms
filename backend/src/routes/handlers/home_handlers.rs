use crate::utils::{
    api_response::{self, ApiResponse},
    app_state::AppState,
};
use actix_web::{get, web, HttpResponse, Responder};
use sea_orm::{ConnectionTrait, Statement};

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
