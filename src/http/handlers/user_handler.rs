use crate::models::User;
use actix_web::web;
use actix_web::{delete, get, post, put, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub full_name: String,
}

#[derive(Serialize)]
pub struct ErrorMessage {
    message: String,
}

#[get("/user/{id}")]
pub async fn find(id: web::Path<i64>, db_pool: web::Data<PgPool>) -> impl Responder {
    log::info!("{}", id);

    let res = User::find(id.into_inner(), &db_pool).await;

    match res {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().json(ErrorMessage {
            message: "User not found.".into(),
        }),
    }
}

#[post("/user")]
pub async fn create(req_body: web::Json<CreateUserRequest>) -> impl Responder {
    HttpResponse::Ok().json(req_body.into_inner())
}

#[put("/user/{id}")]
pub async fn update(_id: web::Path<i64>, req_body: web::Json<CreateUserRequest>) -> impl Responder {
    HttpResponse::Ok().json(req_body.into_inner())
}

#[delete("/user/{id}")]
pub async fn delete(id: web::Path<i64>) -> impl Responder {
    HttpResponse::Ok().body(format!("{}", id))
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(find);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}
