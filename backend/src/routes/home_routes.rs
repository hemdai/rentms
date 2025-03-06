use actix_web::web;

use super::handlers;
use crate::utils::constants::MEDIA_DIRECTORY;
use actix_files::Files;

pub fn config_home(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api/v1")
            .service(handlers::home_handlers::great)
            .service(handlers::home_handlers::hello)
            .service(handlers::home_handlers::get_users)
            .service(handlers::home_handlers::get_rents)
            .service(Files::new("/static/images", MEDIA_DIRECTORY.to_string())),
    );
}
