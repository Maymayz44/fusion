use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use sqlx::{
  PgConnection, postgres::PgRow,
  prelude::FromRow, Row,
  types::Json,
};

use crate::data::queryable::QueryableCode;
use crate::data::Error;
use crate::data::Queryable;

use super::{AuthToken, Source};

#[derive(Serialize, Deserialize)]
pub struct Destination {
  pub id: Option<i32>,
  pub code: String,
  pub path: String,
  pub headers: HashMap<String, String>,
  pub filter: Option<String>,
  pub is_auth: bool,
}

impl Destination {
  pub async fn select_by_path(path: String, conn: &mut PgConnection) -> Result<Self, Error> {
    Ok(sqlx::query_as("
      SELECT destinations.*
      FROM destinations
      WHERE destinations.path = $1
      LIMIT 1;
    ")
    .bind(path)
    .fetch_one(conn)
    .await?)
  }

  pub async fn get_sources(&self, conn: &mut PgConnection) -> Result<Vec<Source>, Error> {
    Ok(sqlx::query_as("
      SELECT sources.*
      FROM destinations
      INNER JOIN destinations__sources
        ON destinations__sources.destination_id = destinations.id
      INNER JOIN sources
        ON sources.id = destinations__sources.source_id
      WHERE destinations.path = $1
      ORDER BY sources.id ASC;
    ")
    .bind(&self.path)
    .fetch_all(conn)
    .await?)
  }

  pub async fn is_token_for(&self, auth_token: &AuthToken, conn: &mut PgConnection) -> Result<bool, Error> {
    match sqlx::query("
      SELECT destinations__auth_tokens.id
      FROM destinations__auth_tokens
      WHERE destinations__auth_tokens.destination_id = $1
        AND destinations__auth_tokens.auth_token_id = $2;
    ")
    .bind(&self.id)
    .bind(&auth_token.id)
    .fetch_optional(conn)
    .await? {
      Some(_) => Ok(auth_token.is_valid()),
      None => Ok(false),
    }
  }

  pub async fn unlink_sources(&self, conn: &mut PgConnection) -> Result<(), Error> {
    sqlx::query("
      DELETE FROM destinations__sources
      WHERE destinations__sources.destination_id = $1;
    ")
    .bind(&self.id)
    .execute(conn)
    .await?;
    
    Ok(())
  }

  pub async fn link_sources(&self, source_codes: Vec<String>, conn: &mut PgConnection) -> Result<(), Error> {
    sqlx::query("
      INSERT INTO destinations__sources (destination_id, source_id)
      SELECT $1 AS destination_id, sources.id AS source_id
      FROM sources
      WHERE sources.code = ANY($2);
    ")
    .bind(&self.id)
    .bind(source_codes)
    .execute(conn)
    .await?;

    Ok(())
  }
}

impl Queryable for Destination {
  async fn select_by_id(id: i32, conn: &mut PgConnection) -> Result<Self, Error> {
    Ok(sqlx::query_as("
      SELECT destinations.*
      FROM destinations
      WHERE destinations.id = $1;
    ")
    .bind(id)
    .fetch_one(conn)
    .await?)
  }

  async fn insert(&self, conn: &mut PgConnection) -> Result<Self, Error> {
    Ok(sqlx::query_as("
      INSERT INTO destinations (code, path, headers, filter, is_auth)
      VALUES ($1, $2, $3, $4, $5)
      RETURNING destinations.*;
    ")
    .bind(&self.code)
    .bind(&self.path)
    .bind(Json(&self.headers))
    .bind(&self.filter)
    .bind(&self.is_auth)
    .fetch_one(conn)
    .await?)
  }

  async fn update(&self, conn: &mut PgConnection) -> Result<Self, Error> {
    Ok(sqlx::query_as("
      UPDATE destinations
      SET path = $1,
          headers = $2,
          filter = $3,
          is_auth = $4
      WHERE destinations.code = $5
      RETURNING destinations.*;
    ")
    .bind(&self.path)
    .bind(Json(&self.headers))
    .bind(&self.filter)
    .bind(&self.is_auth)
    .bind(&self.code)
    .fetch_one(conn)
    .await?)
  }

  async fn delete(&self, conn: &mut PgConnection) -> Result<(), Error> {
    sqlx::query("
      DELETE FROM destinations
      WHERE destinations.code = $1;
    ")
    .bind(&self.code)
    .execute(conn)
    .await?;

    Ok(())
  }

  async fn exists(&self, conn: &mut PgConnection) -> Result<bool, Error> {
    Ok(sqlx::query("SELECT EXISTS(
      SELECT *
      FROM destinations
      WHERE destinations.code = $1
    );")
    .bind(&self.code)
    .fetch_one(conn)
    .await?
    .try_get(0)?)
  }
}

impl QueryableCode for Destination {
  async fn select_by_code(code: String, conn: &mut PgConnection) -> Result<Self, Error> {
    Ok(sqlx::query_as("
      SELECT destinations.*
      FROM destinations
      WHERE destinations.code = $1;
    ")
    .bind(code)
    .fetch_one(conn)
    .await?)
  }
}

impl FromRow<'_, PgRow> for Destination {
  fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
    Ok(Self {
      id: row.try_get("id")?,
      code: row.try_get("code")?,
      path: row.try_get("path")?,
      headers: row.try_get::<Json<HashMap<String, String>>, _>("headers")?.0,
      filter: row.try_get("filter")?,
      is_auth: row.try_get("is_auth")?,
    })
  }
}