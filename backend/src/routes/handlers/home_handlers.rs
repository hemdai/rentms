use actix_web::{web, get, HttpResponse, Responder};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world welcome to lms!")
}

#[get("/users")]
pub async fn get_users() -> impl Responder {
    HttpResponse::Ok().json(vec!["user1", "user2", "user3"])
}
#[get("/rents")]
pub async fn get_rents() -> impl Responder {
    HttpResponse::Ok().json(vec!["rent1", "rent2", "rent3", "rent4", "rent5", "rent6"])
}


#[get("/great/{name}")]
pub async fn great(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}! timro hal khavar k cha bhai", name))
}