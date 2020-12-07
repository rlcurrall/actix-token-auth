use crate::{
    errors::ServiceError,
    models::{PersonalAccessToken, User},
};
use actix_identity::Identity;
use actix_web::{dev::Payload, web::Data, Error, FromRequest, HttpRequest};
use futures::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::pin::Pin;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
    pub full_name: String,
}

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
        let pat_future = PersonalAccessToken::from_request(req, pl);
        let cookie_id = Identity::from_request(req, pl).into_inner();
        let pool = req.app_data::<Data<PgPool>>().unwrap().clone();

        Box::pin(async move {
            let user_id = match pat_future.await {
                Ok(pat) => pat.user_id,
                Err(_) => cookie_id
                    .map_err(|_| ServiceError::Unknown)?
                    .identity()
                    .ok_or(ServiceError::Unauthorized)?
                    .parse::<i64>()
                    .map_err(|_| ServiceError::BadRequest("Invalid cookie.".into()))?,
            };

            let user = Self::find(&pool, user_id).await?;

            Ok(user)
        })
    }
}
