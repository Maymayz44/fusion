use chrono::Utc;
use rand::{distributions::Alphanumeric, Rng};
use sqlx::{postgres::PgRow, prelude::FromRow, types::chrono::DateTime, PgConnection, Row};

use crate::data::{Error, Queryable};

pub struct AuthToken {
  pub id: Option<i32>,
  pub value: String,
  pub expiration: Option<DateTime<Utc>>,
}

impl AuthToken {
  pub fn new(expiration: Option<DateTime<Utc>>) -> Self {
    Self {
      id: None,
      value: rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect(),
      expiration: expiration,
    }
  }

  pub fn is_valid(&self) -> bool {
    match &self.expiration {
      Some(expiration) => expiration > &Utc::now(),
      None => true,
    }
  }

  pub async fn select_by_value(value: String, conn: &mut PgConnection) -> Result<Option<Self>, Error> {
    Ok(sqlx::query_as("
        SELECT auth_tokens.id,
              auth_tokens.value,
              auth_tokens.expiration
        FROM auth_tokens
        WHERE auth_tokens.value = $1;
      ")
      .bind(value)
      .fetch_optional(conn)
      .await?)
  }
}

impl FromRow<'_, PgRow> for AuthToken {
  fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
    Ok(Self {
      id: row.try_get("id")?,
      value: row.try_get("value")?,
      expiration: row.try_get("expiration")?
    })
  }
}

impl Queryable for AuthToken {
  async fn select_by_id(id: i32, conn: &mut sqlx::PgConnection) -> Result<Self, Error>
  where Self: Sized {
    Ok(sqlx::query_as("
        SELECT auth_tokens.id,
              auth_tokens.value,
              auth_tokens.expiration
        FROM auth_tokens
        WHERE auth_tokens.id = $1;
      ")
      .bind(id)
      .fetch_one(conn)
      .await?)
  }

  async fn insert(&self, conn: &mut sqlx::PgConnection) -> Result<(), Error> {
    sqlx::query("
        INSERT INTO auth_tokens (value, expiration)
        VALUES ($1, $2);
      ")
      .bind(&self.value)
      .bind(&self.expiration)
      .execute(conn)
      .await?;

    Ok(())
  }

  async fn update(&self, conn: &mut sqlx::PgConnection) -> Result<(), Error> {
    sqlx::query("
        UPDATE auth_tokens
        SET value = $1,
            expiration = $2
        WHERE auth_tokens.id = $3;
      ")
      .bind(&self.value)
      .bind(&self.expiration)
      .bind(&self.id)
      .execute(conn)
      .await?;
    
    Ok(())
  }

  async fn delete(&self, conn: &mut sqlx::PgConnection) -> Result<(), Error> {
    sqlx::query("
        DELETE FROM auth_tokens
        WHERE auth_tokens.id = $1;
      ")
      .bind(&self.id)
      .execute(conn)
      .await?;

    Ok(())
  }
}