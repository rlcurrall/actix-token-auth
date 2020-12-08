use crate::{
    error::{Result, ServiceError},
    utils::{config::Config, hash},
};
use chrono::{DateTime, Duration, Utc};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::{ser::SerializeStruct, Serialize, Serializer};
use sqlx::{Done, PgPool};
use std::ops::Add;

#[derive(Serialize, Debug)]
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
    pub async fn find_by_token(pool: &PgPool, token: String) -> Result<Self> {
        let transient_token = TransientToken::parse(token)?;

        let pat = sqlx::query_as!(
            Self,
            r#"
                SELECT * FROM personal_access_tokens
                WHERE id = $1
            "#,
            transient_token.id
        )
        .fetch_one(&*pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ServiceError::Unauthorized,
            e => {
                log::error!("Error occurred getting token:\n{}", e);
                ServiceError::Unknown
            }
        })?;

        Ok(pat)
    }

    pub async fn create(
        pool: &PgPool,
        user_id: i64,
        name: String,
        abilities: Option<Vec<String>>,
    ) -> Result<TransientToken> {
        let value: String = thread_rng().sample_iter(&Alphanumeric).take(64).collect();
        let hashed = hash::make(value.clone())?;
        let abilities = abilities.unwrap_or(vec!["*".into()]);

        let token = sqlx::query!(
            r#"
                INSERT INTO personal_access_tokens (user_id, name, token, abilities)
                VALUES ($1, $2, $3, $4)
                RETURNING *
            "#,
            user_id,
            name,
            hashed,
            &abilities
        )
        .fetch_one(&*pool)
        .await
        .map_err(|e| {
            log::error!("Could not create token:\n{}", e);
            ServiceError::Unknown
        })
        .map(|row| TransientToken {
            id: row.id,
            hash: value,
        })?;

        Ok(token)
    }

    pub async fn delete(self, pool: &PgPool) -> Result<u64> {
        let res = sqlx::query!(
            r#"DELETE FROM personal_access_tokens WHERE id = $1"#,
            self.id
        )
        .execute(&*pool)
        .await
        .map_err(|e| {
            log::error!("Could not delete token:\n{}", e);
            ServiceError::Unknown
        })?;

        match res.rows_affected() {
            0 => Err(
                ServiceError::BadRequest("Token does not exist, or already deleted".into()).into(),
            ),
            count => Ok(count),
        }
    }

    pub async fn touch(&self, pool: &PgPool) -> Result<u64> {
        let res = sqlx::query!(
            r#"
            UPDATE personal_access_tokens
            SET last_used_at = now()
            WHERE id = $1"#,
            self.id
        )
        .execute(&*pool)
        .await
        .map_err(|e| {
            log::error!("Could not update token:\n{}", e);
            ServiceError::Unknown
        })?
        .rows_affected();

        Ok(res)
    }

    pub fn verify_token(&self, token: String, config: &Config) -> Result<bool> {
        let transient_token = TransientToken::parse(token)?;

        let expired: bool;
        let valid_hash = hash::check(self.token.clone(), transient_token.hash)?;

        expired = match (config.token_ttl, config.token_refresh) {
            (Some(ttl), true) => Utc::now().ge(&self.last_used_at.add(Duration::minutes(ttl))),
            (Some(ttl), false) => Utc::now().ge(&self.created_at.add(Duration::minutes(ttl))),
            (None, _) => false,
        };

        Ok(valid_hash && !expired)
    }
}

/// Transient Token
///
pub struct TransientToken {
    id: i64,
    hash: String,
}

impl TransientToken {
    pub fn get_type(&self) -> String {
        "bearer".into()
    }

    fn parse(token: String) -> Result<Self> {
        let parse_error =
            ServiceError::InternalServerError("Could not parse authentication token.".into());

        match token.split("|").collect::<Vec<&str>>().as_slice() {
            [id_str, hash_str] => {
                let id = id_str.parse::<i64>().map_err(|_| parse_error)?;
                let hash = hash_str.to_string();

                Ok(TransientToken { id, hash })
            }
            _ => Err(parse_error.into()),
        }
    }
}

impl std::fmt::Display for TransientToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}|{}", self.id, self.hash)
    }
}

impl Serialize for TransientToken {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Color", 2)?;
        state.serialize_field("type", &self.get_type())?;
        state.serialize_field("token", &self.to_string())?;
        state.end()
    }
}
