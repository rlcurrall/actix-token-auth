// use actix_web::{HttpResponse, ResponseError};
// use derive_more::{Display, From};

// #[derive(Display, From, Debug)]
// pub enum RCError {
//     NotFound,
// }
// impl std::error::Error for RCError {}

// impl ResponseError for RCError {
//     fn error_response(&self) -> HttpResponse {
//         match *self {
//             RCError::NotFound => HttpResponse::NotFound().finish(),
//             _ => HttpResponse::InternalServerError().finish(),
//         }
//     }
// }
