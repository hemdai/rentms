use actix_web::web;

use super::handlers;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api/v1")
            .service(handlers::home_handlers::great)
            .service(handlers::home_handlers::hello)
            .service(handlers::home_handlers::get_users)
            .service(handlers::home_handlers::get_rents),
    );
}
