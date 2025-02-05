use crate::routes::middlewares::auth_middleware;
use actix_web::middleware::from_fn;
use actix_web::web;

use super::handlers::post_handlers;

pub fn config_posts(config: &mut web::ServiceConfig) {
    config
        .service(
            web::scope("secure/post")
                .wrap(from_fn(auth_middleware::check_auth_middleware))
                .service(post_handlers::create_post)
                .service(post_handlers::my_posts),
        )
        .service(
            web::scope("/post")
                .service(post_handlers::get_all_post)
                .service(post_handlers::get_post_by_id),
        );
}
