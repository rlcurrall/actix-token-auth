use crate::utils::hash;
use chrono::{DateTime, Utc};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use sqlx::{Done, PgPool};

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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl PersonalAccessToken {
    pub async fn find(token: String, pool: &PgPool) -> sqlx::Result<Self> {
        let invalid_token = sqlx::Error::Decode("invalid token".into());
        let data: Vec<&str> = token.split("|").collect();
        if data.len() != 2 {
            return Err(invalid_token);
        }
        let id = data[0].parse::<i64>().ok();

        let res = sqlx::query!(
            r#"
                SELECT * FROM personal_access_tokens
                WHERE id = $1 AND deleted_at ISNULL
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
            created_at: res.created_at,
            updated_at: res.updated_at,
            deleted_at: res.deleted_at,
        };

        match pat.verify_token(token) {
            true => {
                pat.touch(&pool).await?;
                Ok(pat)
            }
            false => Err(sqlx::Error::RowNotFound),
        }
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
                created_at: res.created_at,
                updated_at: res.updated_at,
                deleted_at: res.deleted_at,
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

    async fn touch(&self, pool: &PgPool) -> sqlx::Result<u64> {
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

    fn verify_token(&self, token: String) -> bool {
        let data: Vec<&str> = token.split("|").collect();

        if data.len() == 2 {
            hash::check(self.token.clone(), data[1].into())
        } else {
            false
        }
    }
}
