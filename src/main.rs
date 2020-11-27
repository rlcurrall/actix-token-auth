mod http;
mod models;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use rcs::{db, hash};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    hash::check("hash: String".into(), "value: String".into());
    dotenv().ok();
    env_logger::Builder::new().parse_env("LOG_LEVEL").init();

    let db_pool = db::get_connection_pool().await;

    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .configure(http::handlers::user_handler::init)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
