pub mod auth {
    use actix_identity::{CookieIdentityPolicy, IdentityService};
    use time::Duration;

    pub fn cookie_auth(config: super::config::Config) -> IdentityService<CookieIdentityPolicy> {
        IdentityService::new(
            CookieIdentityPolicy::new(config.app_key.clone().as_bytes())
                .name("auth")
                .path("/")
                .max_age_time(Duration::days(1))
                .secure(config.app_secure.clone()),
        )
    }
}

pub mod config {
    use std::env;

    #[derive(Clone, Debug)]
    pub struct Config {
        pub app_key: String,
        pub app_domain: String,
        pub app_port: String,
        pub app_address: String,
        pub app_secure: bool,
        pub app_debug: bool,
        pub cors_methods: Vec<String>,
        pub cors_origins: Vec<String>,
        pub cors_credentials: bool,
        pub db_url: String,
        pub token_ttl: Option<i64>,
        pub token_refresh: bool,
    }

    impl Config {
        pub fn init() -> Self {
            Self {
                app_key: env::var("APP_KEY").expect("APP_KEY not set."),
                app_domain: env::var("APP_DOMAIN").unwrap_or("localhost".into()),
                app_port: env::var("APP_PORT").unwrap_or("8080".into()),
                app_address: env::var("APP_ADDRESS").unwrap_or("127.0.0.1".into()),
                app_secure: env::var("APP_SECURE")
                    .unwrap_or("false".into())
                    .parse::<bool>()
                    .expect("Could not convert APP_SECURE to a boolean."),
                app_debug: env::var("APP_DEBUG")
                    .unwrap_or("false".into())
                    .parse::<bool>()
                    .expect("Could not convert APP_DEBUG to a boolean."),
                cors_methods: env::var("CORS_METHODS")
                    .unwrap_or("GET".into())
                    .split(",")
                    .map(|x| x.into())
                    .collect(),
                cors_origins: env::var("CORS_ORIGINS")
                    .unwrap_or("localhost".into())
                    .split(",")
                    .map(|x| x.into())
                    .collect(),
                cors_credentials: env::var("CORS_CREDENTIALS")
                    .unwrap_or("false".into())
                    .parse::<bool>()
                    .expect("Could not convert CORS_CREDENTIALS to a boolean."),
                db_url: env::var("DATABASE_URL").expect("DATABASE_URL is not set."),
                token_ttl: env::var("TOKEN_TTL")
                    .map(|x| {
                        x.parse::<i64>()
                            .expect("Could not convert TOKEN_TTL to integer.")
                    })
                    .ok(),
                token_refresh: env::var("TOKEN_REFRESH")
                    .unwrap_or("false".into())
                    .parse::<bool>()
                    .expect("Could not convert TOKEN_REFRESH to a boolean."),
            }
        }
    }
}

pub mod cors {
    use super::config::Config;
    use actix_cors::Cors;

    pub fn init(config: Config) -> Cors {
        let mut policy = Cors::default().allow_any_header();

        policy = match config.cors_credentials {
            true => policy.supports_credentials(),
            false => policy,
        };

        policy = match config
            .cors_methods
            .iter()
            .map(|x| x.as_str())
            .collect::<Vec<&str>>()
            .as_slice()
        {
            ["*"] => policy.allow_any_method(),
            _ => policy.allowed_methods(config.cors_methods.iter().map(|x| x.as_str())),
        };

        policy = match config
            .cors_origins
            .iter()
            .map(|x| x.as_str())
            .collect::<Vec<&str>>()
            .as_slice()
        {
            ["*"] => policy.allow_any_origin(),
            _ => policy.allowed_origin_fn(move |origin, _req_head| {
                config
                    .cors_origins
                    .iter()
                    .map(|d| origin.as_bytes().ends_with(d.as_bytes()))
                    .fold(false, |acc, x| x || acc)
            }),
        };

        policy
    }
}

pub mod db {
    use sqlx::PgPool;

    pub async fn get_connection_pool(config: super::config::Config) -> PgPool {
        PgPool::connect(&config.db_url)
            .await
            .expect("Could not get database connection pool.")
    }
}

pub mod hash {
    use crate::error::{Result, ServiceError};
    use argon2;
    use std::env;

    pub fn make(value: String) -> Result<String> {
        let hash = argon2::hash_encoded(
            value.as_bytes(),
            env::var("APP_KEY").expect("APP_KEY not set.").as_bytes(),
            &argon2::Config::default(),
        )
        .map_err(|e| {
            log::error!("Could not hash value:\n{}", e);
            ServiceError::Unknown
        })?;

        Ok(hash)
    }

    pub fn check(hash: String, value: String) -> Result<bool> {
        let valid = argon2::verify_encoded(hash.as_str(), value.as_bytes()).map_err(|e| {
            log::error!("Could not hash value:\n{}", e);
            ServiceError::Unknown
        })?;

        Ok(valid)
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
