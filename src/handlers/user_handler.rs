use crate::{error::Result, models::User, requests::user::CreateUser};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path, ServiceConfig},
    HttpResponse, Responder,
};
use sqlx::PgPool;

#[post("/register")]
pub async fn register(data: Json<CreateUser>, pool: Data<PgPool>) -> Result<HttpResponse> {
    let user = User::create(
        &pool,
        data.email.clone(),
        data.password.clone(),
        data.full_name.clone(),
    )
    .await?;

    Ok(HttpResponse::Ok().json(user))
}

#[get("/user/{id}")]
pub async fn find(id: Path<i64>, pool: Data<PgPool>) -> Result<HttpResponse> {
    let user = User::find(&pool, id.into_inner()).await?;

    Ok(HttpResponse::Ok().json(user))
}

#[put("/user/{id}")]
pub async fn update(_: Path<i64>, _: Json<CreateUser>) -> impl Responder {
    HttpResponse::Ok().finish()
}

#[delete("/user/{id}")]
pub async fn delete(id: Path<i64>, pool: Data<PgPool>) -> Result<HttpResponse> {
    User::delete(&pool, id.into_inner()).await?;

    Ok(HttpResponse::NoContent().finish())
}

#[get("/me")]
pub async fn me(me: User) -> impl Responder {
    HttpResponse::Ok().json(me)
}

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(register);
    cfg.service(find);
    cfg.service(update);
    cfg.service(delete);
    cfg.service(me);
}
