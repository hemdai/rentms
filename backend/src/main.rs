use actix_web::{ App, HttpServer};
use actix_web::middleware::Logger;
mod utils;
mod routes;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv::dotenv().ok();
    env_logger::init();

    let address: String = utils::constants::ADDRESS.clone();
    let port: u16 = utils::constants::PORT.clone();

    HttpServer::new(|| {
        App::new().wrap(Logger::default())
        .configure(routes::home_routes::config)
        
    })
    .bind((address, port))?
    .run()
    .await
}


// .route("/", web::get().to(hello))
//         .route("/users", web::get().to(get_users))
//         .route("/rents", web::get().to(get_rents))
//         .route("/great/{name}", web::get().to(great))