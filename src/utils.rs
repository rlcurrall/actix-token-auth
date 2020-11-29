pub mod auth {
    use actix_identity::{CookieIdentityPolicy, IdentityService};
    use time::Duration;

    pub fn cookie_auth() -> IdentityService<CookieIdentityPolicy> {
        let key = super::config::APP_CONFIG.app_key.clone();
        let domain = super::config::APP_CONFIG.app_domain.clone();
        let secure = super::config::APP_CONFIG.app_secure;

        IdentityService::new(
            CookieIdentityPolicy::new(key.as_bytes())
                .name("auth")
                .path("/")
                .domain(domain)
                .max_age_time(Duration::days(1))
                .secure(secure),
        )
    }
}

pub mod config {
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref APP_CONFIG: Config = Config::init();
    }

    #[derive(Clone, Debug)]
    pub struct Config {
        pub app_key: String,
        pub app_domain: String,
        pub app_secure: bool,
        pub app_debug: bool,
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
                app_debug: std::env::var("APP_DEBUG")
                    .unwrap_or("false".into())
                    .parse::<bool>()
                    .unwrap(),
            }
        }
    }
}

pub mod cors {
    pub fn init() -> actix_cors::Cors {
        actix_cors::Cors::default()
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE", "OPTION"])
            .allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b"asdf.test"))
            .max_age(3006)
            .supports_credentials()
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

pub mod log {
    pub fn init_logger() -> Result<(), fern::InitError> {
        let level = match std::env::var("LOG_LEVEL")
            .unwrap_or("error".into())
            .as_str()
        {
            "trace" => log::LevelFilter::Trace,
            "debug" => log::LevelFilter::Debug,
            "info" => log::LevelFilter::Info,
            "warn" => log::LevelFilter::Warn,
            "error" => log::LevelFilter::Error,
            _ => log::LevelFilter::Error,
        };

        fern::Dispatch::new()
            .format(|out, msg, record| {
                out.finish(format_args!(
                    "{}[{}][{}]{}",
                    chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                    record.target(),
                    record.level(),
                    msg
                ))
            })
            .level(level)
            .chain(std::io::stdout())
            .chain(fern::log_file("logs/output.log")?)
            .apply()?;

        Ok(())
    }
}
