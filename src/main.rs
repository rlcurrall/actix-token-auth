mod http;
mod models;

use actix_identity::CookieIdentityPolicy;
use actix_identity::IdentityService;
use actix_web::{middleware, App, HttpServer, web};
use dotenv::dotenv;
use rcs::db;
use time::Duration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = rcs::config::Config::init();
    env_logger::Builder::new().parse_env("LOG_LEVEL").init();

    let db_pool = db::get_connection_pool().await;

    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .wrap(middleware::Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(config.app_key.as_bytes())
                    .name("auth")
                    .path("/")
                    .domain(&config.app_domain)
                    .max_age_time(Duration::days(1))
                    .secure(config.app_secure),
            ))
            .data(web::JsonConfig::default().limit(4096))
            .configure(http::handlers::user_handler::init)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
