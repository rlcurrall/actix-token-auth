use crate::http::errors::ServiceError;
use crate::http::requests::user::{CreateUser, LoginRequest};
use crate::models::User;
use crate::utils::hash;
use actix_identity::Identity;
use actix_web::{
    get, post,
    web::{Data, Json, ServiceConfig},
    HttpResponse, Responder,
};
use sqlx::postgres::PgPool;

#[post("/register")]
pub async fn register(req_body: Json<CreateUser>, db_pool: Data<PgPool>) -> impl Responder {
    let res = User::create(req_body.into_inner(), &db_pool).await;

    match res {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(e) => Err(ServiceError::BadRequest(format!(
            "Could not create user - {}",
            e
        ))),
    }
}

#[post("/login")]
pub async fn login(
    request: Json<LoginRequest>,
    id: Identity,
    db_pool: Data<PgPool>,
) -> impl Responder {
    let res = User::find_by_email(request.email.clone(), &db_pool).await;

    match res {
        Ok(user) => {
            if hash::check(user.password.clone(), request.password.clone()) {
                id.remember(user.id.to_string());
                return Ok(HttpResponse::Ok().finish());
            }

            Err(ServiceError::BadRequest(
                "These credentials do not match our records.".into(),
            ))
        }
        Err(_) => Err(ServiceError::BadRequest(
            "These credentials do not match our records.".into(),
        )),
    }
}

#[get("/logout")]
pub async fn logout(id: Identity) -> impl Responder {
    id.forget();
    HttpResponse::Ok().finish()
}

#[get("/me")]
pub async fn me(me: User) -> impl Responder {
    HttpResponse::Ok().json(me)
}

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(register);
    cfg.service(login);
    cfg.service(logout);
    cfg.service(me);
}
