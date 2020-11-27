pub mod hash {
    use argon2;
    use std::env;

    pub fn make(value: String) -> String {
        argon2::hash_encoded(
            value.as_bytes(),
            env::var("APP_KEY").expect("APP_KEY not set.").as_bytes(),
            &argon2::Config::default(),
        )
        .unwrap()
    }

    pub fn check(hash: String, value: String) -> bool {
        argon2::verify_encoded(hash.as_str(), value.as_bytes()).unwrap()
    }
}

pub mod db {
    use sqlx::PgPool;
    use std::env;

    pub async fn get_connection_pool() -> PgPool {
        let db_uri = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");

        PgPool::connect(&db_uri)
            .await
            .expect("Could not get database connection pool.")
    }
}
