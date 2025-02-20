use super::handlers::property_handlers;
use actix_web::web;
pub fn config_property(config: &mut web::ServiceConfig) {
    config.service(web::scope("/property").service(property_handlers::get_all_property));
}
