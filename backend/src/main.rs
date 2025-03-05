use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use sea_orm::DatabaseConnection;
use utils::app_state::AppState;
mod error;
mod models;
mod routes;
mod services;
mod utils;
// use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=debug");
        env_logger::init();
    }

    dotenv::dotenv().ok();
    let address: String = utils::constants::ADDRESS.clone();
    let port: u16 = utils::constants::PORT.clone();
    let db_url: String = utils::constants::DATABASE_URL.clone();

    let db: DatabaseConnection = Database::connect(db_url)
        .await
        .expect("Database connection error");
    let app_state = web::Data::new(AppState { db: db.clone() });

    Migrator::up(&db, None).await.unwrap();
    // Migrator::fresh(&db);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_header()
                    .allow_any_method()
                    .allow_any_origin()
                    .max_age(36500),
            )
            .configure(routes::home_routes::config_home)
            .configure(routes::auth_routes::config_auth)
            .configure(routes::user_routes::config_users)
            .configure(routes::post_routes::config_posts)
            .configure(routes::property_routes::config_property)
            .configure(routes::country_routes::config_country)
            .configure(routes::address_routes::config_address)
    })
    .bind((address, port))?
    .run()
    .await
}
