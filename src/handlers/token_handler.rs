use crate::{
    errors::ServiceError,
    models::{token::CreateToken, PersonalAccessToken, User},
    requests::token::TokenLogin,
    utils::hash,
};
use actix_web::{
    get, post,
    web::{self, Data, Json, ServiceConfig},
    HttpResponse, Responder,
};
use serde_json::json;
use sqlx::postgres::PgPool;

#[post("/login")]
pub async fn login(req: Json<TokenLogin>, pool: Data<PgPool>) -> impl Responder {
    let res = User::find_by_email(req.email.clone(), &pool).await;

    if let Ok(user) = res {
        if hash::check(user.password.clone(), req.password.clone()) {
            let token_data = CreateToken {
                user_id: user.id,
                name: req.device.clone(),
                abilities: None,
            };
            match PersonalAccessToken::create(token_data, &pool).await {
                Ok((token, _)) => return Ok(HttpResponse::Ok().json(json!({ "token": token }))),
                Err(e) => {
                    log::error!("{}", e);
                    return Err(ServiceError::InternalServerError(
                        "Oops, something went wrong!".into(),
                    ));
                }
            }
        }
    }

    Err(ServiceError::BadRequest(
        "These credentials do not match our records.".into(),
    ))
}

#[get("/logout")]
pub async fn logout(bearer: PersonalAccessToken, pool: Data<PgPool>) -> impl Responder {
    match bearer.delete(&pool).await {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(e) => {
            log::info!("{}", e);
            Err(ServiceError::InternalServerError(
                "Could not delete token.".into(),
            ))
        }
    }
}

#[get("/me")]
pub async fn me(bearer: PersonalAccessToken, pool: Data<PgPool>) -> impl Responder {
    match User::find(bearer.user_id, &pool).await {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => Err(ServiceError::NotFound("User not found.".into())),
    }
}

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/token")
            .service(login)
            .service(logout)
            .service(me),
    );
}
