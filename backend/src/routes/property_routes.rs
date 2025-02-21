use super::handlers::property_handlers;
use crate::routes::middlewares::auth_middleware;
use actix_web::{middleware::from_fn, web};
pub fn config_property(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/property")
            .wrap(from_fn(auth_middleware::check_auth_middleware))
            .service(property_handlers::get_all_property)
            .service(property_handlers::create_property),
    );
}
