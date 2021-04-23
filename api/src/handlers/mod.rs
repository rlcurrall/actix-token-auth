use actix_web::web::{self, ServiceConfig};

mod cookie_handler;
mod token_handler;
mod user_handler;

pub fn init(cfg: &mut ServiceConfig) {
    // Register web routes
    cfg.service(
        web::scope("/")
            .configure(cookie_handler::init)
            .configure(token_handler::init)
            .configure(user_handler::init),
    );

    // Register api routes
    cfg.service(web::scope("/api"));
}
