use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use derive_more::Display;
use serde::Serialize;

#[derive(Serialize)]
struct ErrorMessage {
    error: String,
    message: String,
}

#[derive(Debug, Display)]
#[allow(dead_code)]
pub enum ServiceError {
    #[display(fmt = "{}", _0)]
    BadRequest(String),

    #[display(fmt = "You are unauthorized.")]
    Unauthorized,

    #[display(fmt = "You do not have access to the requested resource.")]
    Forbidden,

    #[display(fmt = "{}", _0)]
    NotFound(String),

    #[display(fmt = "{}", _0)]
    UnprocessableEntity(String),

    #[display(fmt = "Unknown internal server error")]
    Unknown,

    #[display(fmt = "{}", _0)]
    InternalServerError(String),
}

impl ServiceError {
    pub fn name(&self) -> String {
        match *self {
            Self::BadRequest(_) => "Bad Request".into(),
            Self::Unauthorized => "Unauthorized".into(),
            Self::Forbidden => "Forbidden".into(),
            Self::NotFound(_) => "Not Found".into(),
            Self::UnprocessableEntity(_) => "Unprocessable Entity".into(),
            Self::Unknown => "Unknown".into(),
            Self::InternalServerError(_) => "Internal Server Error".into(),
        }
    }
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error = self.name();
        let message = self.to_string();
        HttpResponse::build(status_code).json(ErrorMessage { error, message })
    }
}
