use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_web::middleware::Logger;

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world welcome to lms!")
}

async fn get_users() -> impl Responder {
    HttpResponse::Ok().json(vec!["user1", "user2", "user3"])
}

async fn get_rents() -> impl Responder {
    HttpResponse::Ok().json(vec!["rent1", "rent2", "rent3", "rent4", "rent5", "rent6"])
}


async fn great(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}! timro hal khavar k cha bhai", name))
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv::dotenv().ok();
    env_logger::init();

    HttpServer::new(|| {
        App::new().wrap(Logger::default())
        .route("/", web::get().to(hello))
        .route("/users", web::get().to(get_users))
        .route("/rents", web::get().to(get_rents))
        .route("/great/{name}", web::get().to(great))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}