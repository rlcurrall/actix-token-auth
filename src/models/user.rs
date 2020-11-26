use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use futures::future::{ready, Ready};
use serde::Serialize;
use sqlx::{FromRow, PgPool};

#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: i64,
    pub email: String,
    #[serde(skip)]
    pub password: String,
    pub full_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Responder for User {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        // create response and set content type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

impl User {
    // fn from(row: PgRow) -> Self {
    //     Self {
    //         id: row.id,
    //         email: row.email
    //     }
    // }

    pub async fn find(id: i64, pool: &PgPool) -> sqlx::Result<User> {
        let res = sqlx::query!(
            r#"
                SELECT *
                FROM users
                WHERE id = $1 AND deleted_at ISNULL
            "#,
            id
        )
        .fetch_one(&*pool)
        .await?;

        Ok(User {
            id: res.id,
            email: res.email,
            password: res.password,
            full_name: res.full_name,
            created_at: res.created_at,
            updated_at: res.updated_at,
            deleted_at: res.deleted_at,
        })
    }
}
