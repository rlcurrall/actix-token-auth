pub mod config {
    #[derive(Clone)]
    pub struct Config {
        pub app_key: String,
        pub app_domain: String,
        pub app_secure: bool,
    }

    impl Config {
        pub fn init() -> Self {
            Self {
                app_key: std::env::var("APP_KEY").expect("APP_KEY not set."),
                app_domain: std::env::var("APP_DOMAIN").expect("APP_DOMAIN not set."),
                app_secure: std::env::var("APP_SECURE")
                    .expect("APP_SECURE not set.")
                    .parse::<bool>()
                    .unwrap(),
            }
        }
    }
}

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
