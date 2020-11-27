use crate::models::user::{CreateUser, User};
use actix_web::web;
use actix_web::{delete, get, post, put, HttpResponse, Responder};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct ErrorMessage {
    message: String,
}

#[get("/user/{id}")]
pub async fn find(id: web::Path<i64>, db_pool: web::Data<PgPool>) -> impl Responder {
    let res = User::find(id.into_inner(), &db_pool).await;

    match res {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().json(ErrorMessage {
            message: "User not found.".into(),
        }),
    }
}

#[post("/user")]
pub async fn create(req_body: web::Json<CreateUser>, db_pool: web::Data<PgPool>) -> impl Responder {
    let res = User::create(req_body.into_inner(), &db_pool).await;

    match res {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().json(ErrorMessage {
            message: format!("Could not create user: {}", e),
        }),
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
        Ok(0) => HttpResponse::NotFound().json(ErrorMessage {
            message: "User not found, could not delete.".into(),
        }),
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().json(ErrorMessage {
            message: format!("Could not create user: {}", e),
        }),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(find);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}
