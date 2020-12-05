use crate::{errors::ServiceError, models::User};
use actix_identity::Identity;
use actix_web::{dev::Payload, web::Data, Error, FromRequest, HttpRequest};
use futures::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::pin::Pin;

pub use crate::models::user::CreateUser;

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

impl FromRequest for User {
    type Config = ();
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<User, Error>>>>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        let cookie_id = Identity::from_request(req, pl).into_inner();
        let pool = req.app_data::<Data<PgPool>>().unwrap().clone();

        Box::pin(async move {
            let identity = match cookie_id {
                Ok(identity) => identity,
                Err(_) => return Err(ServiceError::Unauthorized.into()),
            };

            let id = match identity.identity() {
                Some(id_str) => id_str.parse::<i64>().unwrap(),
                None => {
                    identity.forget();
                    return Err(ServiceError::Unauthorized.into());
                }
            };

            match Self::find(id, &pool).await {
                Ok(user) => Ok(user),
                Err(msg) => {
                    identity.forget();
                    log::error!("{}", msg);
                    Err(ServiceError::InternalServerError("".into()).into())
                }
            }
        })
    }
}
