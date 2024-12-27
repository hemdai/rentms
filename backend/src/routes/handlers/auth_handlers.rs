use actix_web::{post,get, web, Responder};
use crate::utils::app_state;
use crate::utils::api_response;
use sea_orm::{Condition, Set,ColumnTrait,EntityTrait,QueryFilter,ActiveModelTrait};
use entity::user::Column;
#[derive(serde::Deserialize)]
struct RegisterModel {
    name: String,
    password: String,
    email: String
}

#[derive(serde::Deserialize)]
struct LoginModel {
    email: String,
    password: String
}

#[get("/auth")]
pub async fn hello_user() -> impl Responder {
    api_response::ApiResponse::new(200, "hello Welcome to hello user !".to_string())
}

#[post("/register")]
pub async fn register(app_state: web::Data<app_state::AppState>, register_json: web::Json<RegisterModel>) -> impl Responder {
    println!("{}", register_json.name);
    let user_model = entity::user::ActiveModel {
        name: Set(register_json.name.clone()),
        email: Set(register_json.email.clone()),
        password: Set(register_json.password.clone()),
        ..Default::default()
    }.insert(&app_state.db).await.unwrap();
    api_response::ApiResponse::new(200, format!("User {} created successfully", user_model.id).to_string())
}


#[post("/get-user")]
pub async fn get_user(app_state: web::Data<app_state::AppState>, login_json: web::Json<LoginModel>) -> impl Responder {
    
    let user_data = entity::user::Entity::find().filter(
            Condition::all()
            .add(Column::Email.eq(&login_json.email))
            .add(Column::Password.eq(&login_json.password))
            ).one(&app_state.db).await.unwrap();
            
    if user_data.is_none() {
        return api_response::ApiResponse::new(400, "User not found".to_string());
    }
    api_response::ApiResponse::new(200, user_data.unwrap().name.to_string())
}


// #[post("/login")]
// pub async fn login(app_state: web::Data<app_state::AppState>, login_json: web::Json<LoginModel>) -> impl Responder {
    
//     let user_data = entity::user::Entity::find().filter(
//             Condition::all()
//             .add(entity::user::Column::Email.eq(&login_json.email))
//             .add(entity::user::Column::Password.eq(&login_json.password))
//             ).one(&app_state.db).await.unwrap();
//     if user_data.is_none() {
//         return api_response::ApiResponse::new(400, "User not found".to_string());
//     }
//     api_response::ApiResponse::new(200, user.unwrap().name.to_string())
// }


