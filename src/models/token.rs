use crate::utils::{config::Config, hash};
use chrono::{DateTime, Duration, Utc};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use sqlx::{Done, PgPool};
use std::ops::Add;

pub struct CreateToken {
    pub user_id: i64,
    pub name: String,
    pub abilities: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct PersonalAccessToken {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    #[serde(skip)]
    pub token: String,
    pub abilities: Option<Vec<String>>,
    pub last_used_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl PersonalAccessToken {
    pub async fn find_by_token(token: String, pool: &PgPool) -> sqlx::Result<Self> {
        let invalid_token = sqlx::Error::Decode("invalid token".into());
        let data: Vec<&str> = token.split("|").collect();
        if data.len() != 2 {
            return Err(invalid_token);
        }
        let id = data[0].parse::<i64>().ok();

        let res = sqlx::query!(
            r#"
                SELECT * FROM personal_access_tokens
                WHERE id = $1
            "#,
            id
        )
        .fetch_one(&*pool)
        .await?;

        let pat = Self {
            id: res.id,
            user_id: res.user_id,
            name: res.name,
            token: res.token,
            abilities: res.abilities,
            last_used_at: res.last_used_at,
            created_at: res.created_at,
        };

        Ok(pat)
    }

    pub async fn create(data: CreateToken, pool: &PgPool) -> sqlx::Result<(String, Self)> {
        let token: String = thread_rng().sample_iter(&Alphanumeric).take(64).collect();
        let hashed = hash::make(token.clone());
        let abilities = data.abilities.unwrap_or(vec!["*".into()]);

        let res = sqlx::query!(
            r#"
                INSERT INTO personal_access_tokens (user_id, name, token, abilities)
                VALUES ($1, $2, $3, $4)
                RETURNING *
            "#,
            data.user_id,
            data.name,
            hashed,
            &abilities
        )
        .fetch_one(&*pool)
        .await?;

        Ok((
            format!("{}|{}", res.id, token),
            Self {
                id: res.id,
                user_id: res.user_id,
                name: res.name,
                token: res.token,
                abilities: res.abilities,
                last_used_at: res.last_used_at,
                created_at: res.created_at,
            },
        ))
    }

    pub async fn delete(self, pool: &PgPool) -> sqlx::Result<u64> {
        let res = sqlx::query!(
            r#"DELETE FROM personal_access_tokens WHERE id = $1"#,
            self.id
        )
        .execute(&*pool)
        .await?
        .rows_affected();

        Ok(res)
    }

    pub async fn touch(&self, pool: &PgPool) -> sqlx::Result<u64> {
        let res = sqlx::query!(
            r#"
            UPDATE personal_access_tokens
            SET last_used_at = now()
            WHERE id = $1"#,
            self.id
        )
        .execute(&*pool)
        .await?
        .rows_affected();

        Ok(res)
    }

    pub fn verify_token(&self, token: String, config: &Config) -> bool {
        let data: Vec<&str> = token.split("|").collect();

        if data.len() == 2 {
            let expired: bool;
            let valid_hash = hash::check(self.token.clone(), data[1].into());

            expired = match (config.token_ttl, config.token_refresh) {
                (Some(ttl), true) => Utc::now().ge(&self.last_used_at.add(Duration::minutes(ttl))),
                (Some(ttl), false) => Utc::now().ge(&self.created_at.add(Duration::minutes(ttl))),
                (None, _) => false,
            };

            valid_hash && !expired
        } else {
            false
        }
    }
}
