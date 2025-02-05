use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use sea_orm::DatabaseConnection;
use utils::app_state::AppState;
mod routes;
mod utils;

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
            .configure(routes::home_routes::config)
            .configure(routes::auth_routes::config_auth)
            .configure(routes::user_routes::config_users)
            .configure(routes::post_routes::config_posts)
    })
    .bind((address, port))?
    .run()
    .await
}
