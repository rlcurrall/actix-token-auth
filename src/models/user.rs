use crate::utils::hash;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Done, FromRow, PgPool};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
    pub full_name: String,
}

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

    pub async fn find_by_email(email: String, pool: &PgPool) -> sqlx::Result<User> {
        let res = sqlx::query!(
            r#"
                SELECT *
                FROM users
                WHERE email = $1 AND deleted_at ISNULL
            "#,
            email
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

    pub async fn create(data: CreateUser, pool: &PgPool) -> sqlx::Result<Self> {
        let password = hash::make(data.password);
        let res = sqlx::query!(
            r#"
                INSERT INTO users (email, password, full_name)
                VALUES ($1, $2, $3)
                RETURNING *
            "#,
            data.email,
            password,
            data.full_name
        )
        .fetch_one(&*pool)
        .await?;

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

    pub async fn delete(id: i64, pool: &PgPool) -> sqlx::Result<u64> {
        let deleted = sqlx::query!("DELETE FROM users WHERE id = $1", id)
            .execute(&*pool)
            .await?
            .rows_affected();

        Ok(deleted)
    }
}
