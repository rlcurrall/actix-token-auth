use crate::{
    error::{Result, ServiceError},
    models::{PersonalAccessToken, User},
    requests::token::TokenLogin,
    utils::{config::Config, hash},
};
use actix_web::{
    cookie::Cookie,
    get, post,
    web::{self, Data, Json, ServiceConfig},
    HttpMessage, HttpRequest, HttpResponse, Responder,
};
use sqlx::PgPool;

#[get("/cookie")]
pub async fn set_cookie(config: Data<Config>) -> impl Responder {
    HttpResponse::Ok()
        .cookie(
            Cookie::build("token", "")
                .http_only(true)
                .secure(config.app_secure)
                .finish(),
        )
        .finish()
}

#[post("/login")]
pub async fn login(req: HttpRequest, data: Json<TokenLogin>, pool: Data<PgPool>) -> impl Responder {
    let user = User::find_by_email(&pool, data.email.clone()).await?;

    if !hash::check(user.password.clone(), data.password.clone())? {
        return Err(ServiceError::BadRequest(
            "These credentials do not match our records.".into(),
        ));
    }

    let transient_token =
        PersonalAccessToken::create(&pool, user.id, data.device.clone(), None).await?;

    let mut response = HttpResponse::Ok();

    if req.cookie("token").is_some() {
        response.cookie(
            Cookie::build("token", transient_token.to_string())
                .http_only(true)
                .finish(),
        );
    }

    Ok(response.json(transient_token))
}

#[get("/logout")]
pub async fn logout(
    req: HttpRequest,
    bearer: PersonalAccessToken,
    pool: Data<PgPool>,
) -> Result<HttpResponse> {
    bearer.delete(&pool).await?;

    let mut res = HttpResponse::NoContent();

    if let Some(ref cookie) = req.cookie("token") {
        res.del_cookie(cookie);
    }

    Ok(res.finish())
}

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/token")
            .service(set_cookie)
            .service(login)
            .service(logout),
    );
}
