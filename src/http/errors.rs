use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use serde::Serialize;

#[derive(Serialize)]
struct ErrorMessage {
    message: String,
}

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error - {}", _0)]
    InternalServerError(String),

    #[display(fmt = "Bad Request - {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized")]
    Unauthorized,

    #[display(fmt = "Forbidden")]
    Forbidden,

    #[display(fmt = "Not Found - {}", _0)]
    NotFound(String),

    #[display(fmt = "Unprocessable Entity - {}", _0)]
    UnprocessableEntity(String),
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError(ref message) => HttpResponse::InternalServerError()
                .json(ErrorMessage {
                    message: message.to_string(),
                }),
            ServiceError::BadRequest(ref message) => {
                HttpResponse::BadRequest().json(ErrorMessage {
                    message: message.to_string(),
                })
            }
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json(ErrorMessage {
                message: "Unauthorized".into(),
            }),
            ServiceError::Forbidden => HttpResponse::Forbidden().json(ErrorMessage {
                message: "Forbidden".into(),
            }),
            ServiceError::NotFound(ref message) => HttpResponse::NotFound().json(ErrorMessage {
                message: message.to_string(),
            }),
            ServiceError::UnprocessableEntity(ref message) => HttpResponse::UnprocessableEntity()
                .json(ErrorMessage {
                    message: message.to_string(),
                }),
        }
    }
}
