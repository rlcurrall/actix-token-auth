mod http;
mod models;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;

async fn get_database_pool() -> PgPool {
    let db_uri = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");

    PgPool::connect(&db_uri)
        .await
        .expect("Could not get database connection pool.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::Builder::new().parse_env("LOG_LEVEL").init();

    let db_pool = get_database_pool().await;

    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .configure(http::handlers::user_handler::init)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
