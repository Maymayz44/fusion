use chrono::Utc;
use rand::{distributions::Alphanumeric, Rng};
use sqlx::{postgres::PgRow, prelude::FromRow, types::chrono::DateTime, PgConnection, Row};

use crate::{data::{Error, Queryable}, utils::Hasher};

pub struct AuthToken {
  pub id: Option<i32>,
  pub value: Vec<u8>,
  pub expiration: Option<DateTime<Utc>>,
}

impl AuthToken {
  pub fn new(expiration: Option<DateTime<Utc>>) -> Self {
    Self {
      id: None,
      value: Hasher::hash_string(
        rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
      ),
      expiration: expiration,
    }
  }

  pub fn is_valid(&self) -> bool {
    match &self.expiration {
      Some(expiration) => expiration > &Utc::now(),
      None => true,
    }
  }

  pub async fn select_by_value(value: Vec<u8>, conn: &mut PgConnection) -> Result<Option<Self>, Error> {
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

  pub async fn unlink_destinations(&self, conn: &mut PgConnection) -> Result<(), Error> {
    sqlx::query("
      DELETE FROM destinations__auth_tokens
      WHERE destinations__auth_tokens.auth_token_id = $1;
    ")
    .bind(&self.id)
    .execute(conn)
    .await?;
    
    Ok(())
  }

  pub async fn link_destinations(&self, destination_codes: Vec<String>, conn: &mut PgConnection) -> Result<(), Error> {
    sqlx::query("
      INSERT INTO destinations__auth_tokens (destination_id, auth_token_id)
      SELECT destinations.id AS destination_id, $1 AS auth_token_id
      FROM destinations
      WHERE destinations.code = ANY($2);
    ")
    .bind(&self.id)
    .bind(destination_codes)
    .execute(conn)
    .await?;

    Ok(())
  }
}

impl Queryable for AuthToken {
  async fn select_by_id(id: i32, conn: &mut PgConnection) -> Result<Self, Error> {
    Ok(sqlx::query_as("
      SELECT auth_tokens.*
      FROM auth_tokens
      WHERE auth_tokens.id = $1;
    ")
    .bind(id)
    .fetch_one(conn)
    .await?)
  }

  async fn insert(&self, conn: &mut PgConnection) -> Result<Self, Error> {
    Ok(sqlx::query_as("
      INSERT INTO auth_tokens (value, expiration)
      VALUES ($1, $2)
      RETURNING auth_tokens.*;
    ")
    .bind(&self.value)
    .bind(&self.expiration)
    .fetch_one(conn)
    .await?)
  }

  async fn update(&self, conn: &mut PgConnection) -> Result<Self, Error> {
    Ok(sqlx::query_as("
      UPDATE auth_tokens
      SET value = $1,
          expiration = $2
      WHERE auth_tokens.id = $3
      RETURNING auth_tokens.*;
    ")
    .bind(&self.value)
    .bind(&self.expiration)
    .bind(&self.id)
    .fetch_one(conn)
    .await?)
  }

  async fn delete(&self, conn: &mut PgConnection) -> Result<(), Error> {
    sqlx::query("
      DELETE FROM auth_tokens
      WHERE auth_tokens.id = $1;
    ")
    .bind(&self.id)
    .execute(conn)
    .await?;

    Ok(())
  }

  async fn exists(&self, conn: &mut PgConnection) -> Result<bool, Error> {
    Ok(sqlx::query("SELECT EXISTS(
      SELECT *
      FROM auth_tokens
      WHERE auth_tokens.value = $1;
    )")
    .bind(&self.value)
    .fetch_one(conn)
    .await?
    .try_get(0)?)
  }

  async fn insert_or_update(&self, conn: &mut PgConnection) -> Result<Self, Error> {
    Ok(match Self::select_by_value(self.value.clone(), conn).await? {
      Some(token) => token.update(conn).await?,
      None => self.insert(conn).await?,
    })
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