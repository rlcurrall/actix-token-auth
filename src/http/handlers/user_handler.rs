use crate::http::{
    errors::ServiceError,
    requests::user::{CreateUser, LoginRequest},
};
use crate::models::User;
use crate::utils::hash;
use actix_identity::Identity;
use actix_web::web;
use actix_web::{delete, get, post, put, HttpResponse, Responder};
use sqlx::PgPool;

#[get("/user/{id}")]
pub async fn find(id: web::Path<i64>, db_pool: web::Data<PgPool>) -> impl Responder {
    let res = User::find(id.into_inner(), &db_pool).await;

    match res {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => Err(ServiceError::BadRequest("User not found.".into())),
    }
}

#[post("/login")]
pub async fn login(
    request: web::Json<LoginRequest>,
    id: Identity,
    db_pool: web::Data<PgPool>,
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

#[post("/register")]
pub async fn create(req_body: web::Json<CreateUser>, db_pool: web::Data<PgPool>) -> impl Responder {
    let res = User::create(req_body.into_inner(), &db_pool).await;

    match res {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(e) => Err(ServiceError::BadRequest(format!(
            "Could not create user: {}",
            e
        ))),
    }
}

#[put("/user/{id}")]
pub async fn update(_id: web::Path<i64>, req_body: web::Json<CreateUser>) -> impl Responder {
    HttpResponse::Ok().json(req_body.into_inner())
}

#[delete("/user/{id}")]
pub async fn delete(id: web::Path<i64>, db_pool: web::Data<PgPool>) -> impl Responder {
    let res = User::delete(id.into_inner(), &db_pool).await;

    match res {
        Ok(1) => Ok(HttpResponse::NoContent().finish()),
        Ok(0) => Err(ServiceError::BadRequest(
            "User not found, could not delete.".into(),
        )),
        Err(e) => Err(ServiceError::BadRequest(format!(
            "Could not create user - {}",
            e
        ))),
        _ => Err(ServiceError::InternalServerError("".into())),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(find);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
    cfg.service(login);
    cfg.service(logout);
    cfg.service(me);
}
