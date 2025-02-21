use super::handlers::country_handlers;
use crate::routes::middlewares::auth_middleware;
use actix_web::{middleware::from_fn, web};

pub fn config_country(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/country")
            .wrap(from_fn(auth_middleware::check_auth_middleware))
            .service(country_handlers::create_country)
            .service(country_handlers::get_all_country),
    );
}
