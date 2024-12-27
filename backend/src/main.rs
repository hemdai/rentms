use actix_web::{ App, HttpServer, web};
use actix_web::middleware::Logger;
use sea_orm::DatabaseConnection;
use sea_orm::Database;
use utils::app_state::AppState;
use migration::{Migrator, MigratorTrait};
mod utils;
mod routes;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=debug");
        env_logger::init();
    }
    dotenv::dotenv().ok();

    let address: String = utils::constants::ADDRESS.clone();
    let port: u16 = utils::constants::PORT.clone();
    let db_url: String = utils::constants::DATABASE_URL.clone();

    let db: DatabaseConnection = Database::connect(db_url).await.expect("Database connection error");
    let app_state = web::Data::new(AppState { db: db.clone() });

    Migrator::up(&db, None).await.unwrap();

    HttpServer::new(move|| {
        App::new()
            .app_data(app_state.clone())
        .wrap(Logger::default())
        .configure(routes::home_routes::config)
        .configure(routes::auth_routes::config_auth)
        
    })
    .bind((address, port))?
    .run()
    .await
}
