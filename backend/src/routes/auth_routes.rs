use super::handlers;
use actix_web::web;

pub fn config_auth(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/auth")
            .service(handlers::auth_handlers::register)
            .service(handlers::auth_handlers::get_user)
            .service(handlers::auth_handlers::hello_user),
    );
}
// .service(handlers::auth_handlers::login),
