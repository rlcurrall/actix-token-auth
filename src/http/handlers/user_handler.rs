use crate::http::{errors::ServiceError, requests::user::CreateUser as UpdateUser};
use crate::models::User;
use actix_web::{
    delete, get, put,
    web::{Data, Json, Path, ServiceConfig},
    HttpResponse, Responder,
};
use sqlx::PgPool;

#[get("/user/{id}")]
pub async fn find(id: Path<i64>, db_pool: Data<PgPool>) -> impl Responder {
    let res = User::find(id.into_inner(), &db_pool).await;

    match res {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => Err(ServiceError::BadRequest("User not found.".into())),
    }
}

#[put("/user/{id}")]
pub async fn update(_id: Path<i64>, req_body: Json<UpdateUser>) -> impl Responder {
    HttpResponse::Ok().json(req_body.into_inner())
}

#[delete("/user/{id}")]
pub async fn delete(id: Path<i64>, db_pool: Data<PgPool>) -> impl Responder {
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

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(find);
    cfg.service(update);
    cfg.service(delete);
}
