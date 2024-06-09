use std::collections::HashMap;
use rocket::serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{
  PgConnection,
  prelude::FromRow,
  types::Json,
  Row,
};

use crate::data::Error;
use crate::data::Queryable;

use super::Source;

#[derive(Serialize, Deserialize)]
pub struct Destination {
  pub id: Option<i32>,
  pub path: String,
  pub headers: HashMap<String, String>,
  pub filter: Option<String>,
}

impl FromRow<'_, PgRow> for Destination {
  fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
    Ok(Self {
      id: row.try_get("id")?,
      path: row.try_get("path")?,
      headers: row.try_get::<Json<HashMap<String, String>>, _>("headers")?.0,
      filter: row.try_get("filter")?,
    })
  }
}

impl Destination {
  pub async fn select_by_path(path: String, conn: &mut PgConnection) -> Result<Self, Error> {
    Ok(sqlx::query_as("
        SELECT destinations.id,
               destinations.path,
               destinations.headers,
               destinations.filter
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
        FROM destinations_sources
        INNER JOIN sources
          ON sources.id = destinations_sources.source_id
        WHERE destinations_sources.destination_id = $1
        ORDER BY sources.id ASC
      ")
      .bind(&self.id)
      .fetch_all(conn)
      .await?)
  }
}

impl Queryable for Destination {
  async fn select_by_id(id: i32, conn: &mut PgConnection) -> Result<Self, Error>
  where Self: Sized {
    Ok(sqlx::query_as("
        SELECT destinations.id,
               destinations.path,
               destinations.headers,
               destinations.filter
        FROM destinations
        WHERE destinations.id = $1
      ")
      .bind(id)
      .fetch_one(conn)
      .await?)
  }

  async fn insert(&self, conn: &mut PgConnection) -> Result<(), Error> {
    sqlx::query("
        INSERT INTO destinations (path, headers, filter)
        VALUES ($1, $2, $3)
      ")
      .bind(&self.path)
      .bind(Json(&self.headers))
      .bind(&self.filter)
      .execute(conn)
      .await?;

    Ok(())
  }

  async fn update(&self, conn: &mut PgConnection) -> Result<(), Error> {
    sqlx::query("
        UPDATE destinations
        SET headers = $1,
            filter = $2
        WHERE destinations.path = $8
      ")
      .bind(Json(&self.headers))
      .bind(&self.filter)
      .bind(&self.path)
      .execute(conn)
      .await?;

    Ok(())
  }

  async fn delete(&self, conn: &mut PgConnection) -> Result<(), Error> {
    sqlx::query("
        DELETE FROM destinations
        WHERE destinations.path = $1
      ")
      .bind(&self.path)
      .execute(conn)
      .await?;

    Ok(())
  }
}