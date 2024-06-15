use chrono::{Utc};
use rand::{distributions::Alphanumeric, Rng};
use rocket::request::{FromRequest, Outcome, Request};
use sqlx::{postgres::PgRow, prelude::FromRow, Row, types::chrono::DateTime};

use crate::data::Queryable;

pub struct AuthToken {
  pub id: Option<i32>,
  pub token: String,
  pub expiration: Option<DateTime<Utc>>,
}

impl AuthToken {
  pub fn new(expiration: Option<DateTime<Utc>>) -> Self {
    Self {
      id: None,
      token: rand::thread_rng()
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
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthToken {
  type Error = crate::api::Error;

  async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    let path = request.uri().path().to_string()[4..].to_owned();
    
    Outcome::Success(Self {
      id: None,
      token: request.headers().get_one("Authorization").unwrap_or("").to_owned(),
      expiration: None,
    })
  }
}

impl FromRow<'_, PgRow> for AuthToken {
  fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
      Ok(Self {
        id: row.try_get("id")?,
        token: row.try_get("token")?,
        expiration: row.try_get("expiration")?
      })
  }
}

impl Queryable for AuthToken {
  async fn select_by_id(id: i32, conn: &mut sqlx::PgConnection) -> Result<Self, crate::data::Error>
  where Self: Sized {
    Ok(sqlx::query_as("
        SELECT auth_tokens.id,
              auth_tokens.token,
              auth_tokens.expiration
        FROM auth_tokens
        WHERE auth_tokens.id = $1;
      ")
      .bind(id)
      .fetch_one(conn)
      .await?)
  }

  async fn insert(&self, conn: &mut sqlx::PgConnection) -> Result<(), crate::data::Error> {
    sqlx::query("
        INSERT INTO auth_tokens (token, expiration)
        VALUES ($1, $2);
      ")
      .bind(&self.token)
      .bind(&self.expiration)
      .execute(conn)
      .await?;

    Ok(())
  }

  async fn update(&self, conn: &mut sqlx::PgConnection) -> Result<(), crate::data::Error> {
    sqlx::query("
        UPDATE auth_tokens
        SET token = $1,
            expiration = $2
        WHERE auth_tokens.id = $3;
      ")
      .bind(&self.token)
      .bind(&self.expiration)
      .bind(&self.id)
      .execute(conn)
      .await?;
    
    Ok(())
  }

  async fn delete(&self, conn: &mut sqlx::PgConnection) -> Result<(), crate::data::Error> {
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