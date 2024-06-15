use std::collections::HashMap;
use chrono::Utc;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{
  PgConnection,
  prelude::FromRow,
  types::Json,
  Row,
};

use crate::data::{Error, POOL};
use crate::data::Queryable;
use crate::api::Error as ApiError;

use super::{AuthToken, Source};

#[derive(Serialize, Deserialize)]
pub struct Destination {
  pub id: Option<i32>,
  pub path: String,
  pub headers: HashMap<String, String>,
  pub filter: Option<String>,
  pub is_auth: bool,
}

impl Destination {
  pub async fn select_by_path(path: String, conn: &mut PgConnection) -> Result<Self, Error> {
    Ok(sqlx::query_as("
        SELECT destinations.id,
               destinations.path,
               destinations.headers,
               destinations.filter,
               destinations.is_auth
        FROM destinations
        WHERE destinations.path = $1
      ")
      .bind(path)
      .fetch_one(conn)
      .await?)
  }

  pub async fn get_sources(&self, conn: &mut PgConnection) -> Result<Vec<Source>, Error> {
    Ok(sqlx::query_as("
        SELECT sources.id,
               sources.url,
               sources.body,
               sources.params,
               sources.headers,
               sources.auth_type,
               sources.auth_username,
               sources.auth_password,
               sources.auth_token
        FROM destinations
        INNER JOIN destinations__sources
          ON destinations__sources.destination_id = destinations.id
        INNER JOIN sources
          ON sources.id = destinations__sources.source_id
        WHERE destinations.path = $1
        ORDER BY sources.id ASC
      ")
      .bind(&self.path)
      .fetch_all(conn)
      .await?)
  }

  pub async fn get_valid_tokens(&self, conn: &mut PgConnection) -> Result<Vec<AuthToken>, Error> {
    Ok(sqlx::query_as("
        SELECT auth_tokens.id,
               auth_tokens.token,
               auth_tokens.expiration
        FROM destinations
        INNER JOIN destinations__auth_tokens
          ON destinations__auth_tokens.destination_id = destinations.id
        INNER JOIN auth_tokens
          ON auth_tokens.id = destinations__auth_tokens.auth_token_id
        WHERE destinations.path = $1
          AND auth_tokens.expiration > $2
      ")
      .bind(&self.id)
      .bind(&Utc::now())
      .fetch_all(conn)
      .await?)
  }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Destination {
  type Error = ApiError;

  async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    let mut conn: sqlx::pool::PoolConnection<sqlx::Postgres> = POOL.get()
      .ok_or_else(|| Self::Error::InternalServerError(String::from(""))).unwrap()
      .acquire().await.unwrap();
    let path = request.uri().path().to_string()[4..].to_owned();
  
    let dest = Destination::select_by_path(path, &mut conn).await.unwrap();
    let tokens = dest.get_valid_tokens(&mut conn).await.unwrap();

    if tokens.iter().any(|token| request.headers().get_one("").unwrap_or("").to_owned() == token.token) {
      return Outcome::Success(dest);
    }

    Outcome::Error((Status::Unauthorized, ApiError::Unauthorized(())))
  }
}

impl FromRow<'_, PgRow> for Destination {
  fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
    Ok(Self {
      id: row.try_get("id")?,
      path: row.try_get("path")?,
      headers: row.try_get::<Json<HashMap<String, String>>, _>("headers")?.0,
      filter: row.try_get("filter")?,
      is_auth: row.try_get("is_auth")?,
    })
  }
}

impl Queryable for Destination {
  async fn select_by_id(id: i32, conn: &mut PgConnection) -> Result<Self, Error>
  where Self: Sized {
    Ok(sqlx::query_as("
        SELECT destinations.id,
               destinations.path,
               destinations.headers,
               destinations.filter,
               destinations.is_auth
        FROM destinations
        WHERE destinations.id = $1;
      ")
      .bind(id)
      .fetch_one(conn)
      .await?)
  }

  async fn insert(&self, conn: &mut PgConnection) -> Result<(), Error> {
    sqlx::query("
        INSERT INTO destinations (path, headers, filter, is_auth)
        VALUES ($1, $2, $3, $4);
      ")
      .bind(&self.path)
      .bind(Json(&self.headers))
      .bind(&self.filter)
      .bind(&self.is_auth)
      .execute(conn)
      .await?;

    Ok(())
  }

  async fn update(&self, conn: &mut PgConnection) -> Result<(), Error> {
    sqlx::query("
        UPDATE destinations
        SET headers = $1,
            filter = $2,
            is_auth = $3
        WHERE destinations.path = $4;
      ")
      .bind(Json(&self.headers))
      .bind(&self.filter)
      .bind(&self.is_auth)
      .bind(&self.path)
      .execute(conn)
      .await?;

    Ok(())
  }

  async fn delete(&self, conn: &mut PgConnection) -> Result<(), Error> {
    sqlx::query("
        DELETE FROM destinations
        WHERE destinations.path = $1;
      ")
      .bind(&self.path)
      .execute(conn)
      .await?;

    Ok(())
  }
}