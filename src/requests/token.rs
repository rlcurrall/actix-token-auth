use crate::{errors::ServiceError, models::PersonalAccessToken};
use actix_web::{dev::Payload, web::Data, Error, FromRequest, HttpRequest};
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
        let credentials = BearerAuth::from_request(req, pl).into_inner();

        Box::pin(async move {
            let bearer = credentials
                .map_err(|_| ServiceError::Unauthorized)?
                .token()
                .to_owned();

            let token = Self::find(bearer, &pool)
                .await
                .map_err(|_| ServiceError::Unauthorized)?;

            Ok(token)
        })
    }
}
