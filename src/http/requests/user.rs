use serde::{Deserialize, Serialize};

pub use crate::models::CreateUser;

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
