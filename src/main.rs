mod errors;
mod handlers;
mod models;
mod requests;
mod utils;

use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use utils::{config, db};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = config::Config::init();

    utils::log::init_logger().expect("Failed to initialize logger.");

    let db_pool = db::get_connection_pool(config.clone()).await;

    HttpServer::new(move || {
        App::new()
            .data(config.clone())
            .data(db_pool.clone())
            .wrap(middleware::Logger::default())
            .wrap(utils::cors::init(config.clone()))
            .wrap(utils::auth::cookie_auth(config.clone()))
            .data(web::JsonConfig::default().limit(4096))
            .configure(handlers::user_handler::init)
            .configure(handlers::cookie_handler::init)
            .configure(handlers::token_handler::init)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
