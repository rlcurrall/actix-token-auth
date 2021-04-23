use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use utils::{config, db};

mod error;
mod handlers;
mod models;
mod requests;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    utils::log::init_logger().expect("Failed to initialize logger.");

    let config = config::Config::init();
    let db_pool = db::get_connection_pool(config.clone()).await;
    let address = format!("{}:{}", config.app_address, config.app_port);

    HttpServer::new(move || {
        App::new()
            .data(config.clone())
            .data(db_pool.clone())
            .data(web::JsonConfig::default().limit(4096))
            .wrap(middleware::Logger::default())
            .wrap(utils::cors::init(config.clone()))
            .wrap(utils::auth::cookie_auth(config.clone()))
            .configure(handlers::init)
    })
    .bind(address)?
    .run()
    .await
}
