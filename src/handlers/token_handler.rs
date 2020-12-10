use crate::{
    error::{Result, ServiceError},
    models::{PersonalAccessToken, User},
    requests::token::TokenLogin,
    utils::{config::Config, hash},
};
use actix_web::{
    cookie::{Cookie, CookieBuilder},
    get, post,
    web::{self, Data, Json, ServiceConfig},
    HttpMessage, HttpRequest, HttpResponse, Responder,
};
use sqlx::PgPool;

fn make_cookie(config: &Config, token: Option<String>) -> CookieBuilder<'static> {
    Cookie::build("token", token.unwrap_or("".into()))
        .path("/")
        .http_only(true)
        .secure(config.app_secure)
}

#[get("/cookie")]
pub async fn set_cookie(config: Data<Config>) -> impl Responder {
    HttpResponse::Ok()
        .cookie(make_cookie(&config, None).finish())
        .finish()
}

#[post("/login")]
pub async fn login(
    req: HttpRequest,
    data: Json<TokenLogin>,
    pool: Data<PgPool>,
    config: Data<Config>,
) -> impl Responder {
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
        response.cookie(make_cookie(&config, Some(transient_token.to_string())).finish());
    }

    Ok(response.json(transient_token))
}

#[get("/logout")]
pub async fn logout(
    bearer: PersonalAccessToken,
    pool: Data<PgPool>,
    config: Data<Config>,
) -> Result<HttpResponse> {
    bearer.delete(&pool).await?;

    let mut res = HttpResponse::NoContent();

    res.cookie(
        make_cookie(&config, None)
            .expires(time::OffsetDateTime::now_utc())
            .finish(),
    );

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
