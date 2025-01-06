use crate::routes::middlewares::auth_middleware;
use actix_web::middleware::from_fn;
use actix_web::web;

use super::handlers;

pub fn config_users(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/users")
            .wrap(from_fn(auth_middleware::check_auth_middleware))
            .service(handlers::user_handlers::login_verify)
            .service(handlers::user_handlers::update_user),
    );
}
