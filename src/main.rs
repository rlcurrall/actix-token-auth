// extern crate argon2;

mod http;
mod models;

use actix_web::{App, HttpServer};
use argon2;
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;

async fn get_database_pool() -> PgPool {
    let db_uri = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");

    PgPool::connect(&db_uri)
        .await
        .expect("Could not get database connection pool.")
}

fn hash_make(value: String) -> String {
    argon2::hash_encoded(
        value.as_bytes(),
        env::var("APP_KEY").expect("APP_KEY not set.").as_bytes(),
        &argon2::Config::default(),
    )
    .unwrap()
}

fn hash_check(hash: String, value: String) -> bool {
    argon2::verify_encoded(hash.as_str(), value.as_bytes()).unwrap()
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
