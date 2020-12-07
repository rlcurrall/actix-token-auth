use crate::{errors::ServiceError, models::PersonalAccessToken, utils::config::Config};
use actix_web::{dev::Payload, web::Data, Error, FromRequest, HttpMessage, HttpRequest};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::{future::Future, pin::Pin};

#[derive(Serialize, Deserialize)]
pub struct TokenLogin {
    pub email: String,
    pub password: String,
    pub device: String,
}

impl FromRequest for PersonalAccessToken {
    type Config = ();
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = std::result::Result<PersonalAccessToken, Error>>>>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        let pool = req.app_data::<Data<PgPool>>().unwrap().clone();
        let config = req.app_data::<Data<Config>>().unwrap().clone();
        let from_cookie = req.cookie("token");
        let from_header = BearerAuth::from_request(req, pl).into_inner();

        Box::pin(async move {
            let token_str = match (from_header, from_cookie) {
                (Ok(bearer), _) => bearer.token().to_owned(),
                (_, Some(data)) => data.value().to_owned(),
                _ => return Err(ServiceError::Unauthorized.into()),
            };

            let token = PersonalAccessToken::find_by_token(&pool, token_str.clone()).await?;

            match token.verify_token(token_str.clone(), &config)? {
                false => Err(ServiceError::Unauthorized.into()),
                true => {
                    token
                        .touch(&pool)
                        .await
                        .map_err(|_| ServiceError::Unknown)?;
                    Ok(token)
                }
            }
        })
    }
}
