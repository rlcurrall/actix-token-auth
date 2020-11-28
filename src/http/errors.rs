use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use serde::Serialize;

#[derive(Serialize)]
struct ErrorMessage {
    message: String,
}

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "BadRequest - {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized")]
    Unauthorized,
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json(ErrorMessage {
                    message: "Internal Server Error, please try later.".into(),
                })
            }
            ServiceError::BadRequest(ref message) => {
                HttpResponse::BadRequest().json(ErrorMessage {
                    message: message.into(),
                })
            }
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json(ErrorMessage {
                message: "Unauthorized".into(),
            }),
        }
    }
}
