use crate::{errors::ServiceError, models::User, requests::user::LoginRequest, utils::hash};
use actix_identity::Identity;
use actix_web::{
    get, post,
    web::{self, Data, Json, ServiceConfig},
    HttpResponse, Responder,
};
use sqlx::postgres::PgPool;

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
    cfg.service(
        web::scope("/cookie")
            .service(login)
            .service(logout)
            .service(me),
    );
}
