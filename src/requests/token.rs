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
        let credentials = BearerAuth::from_request(req, pl).into_inner();
        let pool = req.app_data::<Data<PgPool>>().unwrap().clone();

        Box::pin(async move {
            let bearer = match credentials {
                Ok(t) => t.token().to_owned(),
                Err(_) => return Err(ServiceError::Unauthorized.into()),
            };

            let id = match Self::verify(bearer.clone(), &pool).await {
                Ok(false) => return Err(ServiceError::Unauthorized.into()),
                Ok(true) => {
                    let id_parse = bearer.split("|").collect::<Vec<&str>>()[0].parse::<i64>();
                    match id_parse {
                        Err(_) => {
                            return Err(ServiceError::BadRequest("Invalid token".into()).into())
                        }
                        Ok(id) => id,
                    }
                }
                Err(error) => match error {
                    sqlx::Error::RowNotFound => return Err(ServiceError::Unauthorized.into()),
                    _ => return Err(ServiceError::InternalServerError(error.to_string()).into()),
                },
            };

            match Self::find(id, &pool).await {
                Ok(user) => Ok(user),
                Err(_) => Err(ServiceError::InternalServerError("".into()).into()),
            }
        })
    }
}
