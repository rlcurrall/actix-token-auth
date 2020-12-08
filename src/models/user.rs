use crate::{
    error::{Result, ServiceError},
    utils::hash,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Done, FromRow, PgPool};

#[derive(Debug, Serialize, Deserialize, FromRow)]
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

impl User {
    pub async fn find(pool: &PgPool, id: i64) -> Result<User> {
        let res = sqlx::query!(
            r#"
                SELECT *
                FROM users
                WHERE id = $1 AND deleted_at ISNULL
            "#,
            id
        )
        .fetch_one(&*pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ServiceError::NotFound("User not found.".into()),
            _ => {
                log::error!("Could not fetch user:\n{}", e);
                ServiceError::Unknown
            }
        })?;

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

    pub async fn find_by_email(pool: &PgPool, email: String) -> Result<User> {
        let res = sqlx::query!(
            r#"
                SELECT *
                FROM users
                WHERE email = $1 AND deleted_at ISNULL
            "#,
            email
        )
        .fetch_one(&*pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ServiceError::NotFound("User not found.".into()),
            _ => {
                log::error!("Could not fetch user:\n{}", e);
                ServiceError::Unknown
            }
        })?;

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

    pub async fn create(
        pool: &PgPool,
        email: String,
        password: String,
        full_name: String,
    ) -> Result<Self> {
        let password = hash::make(password);
        let res = sqlx::query!(
            r#"
                INSERT INTO users (email, password, full_name)
                VALUES ($1, $2, $3)
                RETURNING *
            "#,
            email,
            password,
            full_name
        )
        .fetch_one(&*pool)
        .await
        .map_err(|e| {
            log::error!("Could not create user:\n{}", e);
            ServiceError::Unknown
        })?;

        Ok(Self {
            id: res.id,
            email: res.email,
            password: res.password,
            full_name: res.full_name,
            created_at: res.created_at,
            updated_at: res.updated_at,
            deleted_at: res.deleted_at,
        })
    }

    pub async fn delete(pool: &PgPool, id: i64) -> Result<u64> {
        let deleted = sqlx::query!("DELETE FROM users WHERE id = $1", id)
            .execute(&*pool)
            .await
            .map_err(|e| {
                log::error!("Could not delete user:\n{}", e);
                ServiceError::Unknown
            })?;

        match deleted.rows_affected() {
            0 => Err(ServiceError::BadRequest(
                "User does not exist, or has been deleted already.".into(),
            )),
            count => Ok(count),
        }
    }
}
