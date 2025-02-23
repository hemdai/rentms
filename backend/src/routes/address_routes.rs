use super::handlers::address_handlers;
use crate::routes::middlewares::auth_middleware;
use actix_web::{middleware::from_fn, web};
pub fn config_address(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/address")
            .wrap(from_fn(auth_middleware::check_auth_middleware))
            .service(address_handlers::get_address)
            .service(address_handlers::create_address),
    );
}
