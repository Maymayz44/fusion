use std::collections::HashMap;
use rocket::serde::{Deserialize, Serialize};
use sqlx::{
  types::Json,
  postgres::PgRow, prelude::FromRow, Row,
};

use crate::data::{Error, Queryable, types::Auth};

#[derive(Serialize, Deserialize)]
pub struct Source {
  pub id: Option<i32>,
  pub url: String,
  pub body: String,
  pub params: HashMap<String, String>,
  pub headers: HashMap<String, String>,
  pub auth: Auth,
}

impl FromRow<'_, PgRow> for Source {
  fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
    Ok(Self {
      id: row.try_get("id")?,
      url: row.try_get("url")?,
      body: row.try_get("body")?,
      params: row.try_get::<Json<HashMap<String, String>>, _>("params")?.0,
      headers: row.try_get::<Json<HashMap<String, String>>, _>("headers")?.0,
      auth: Auth::from_row(row)?,
    })
  }
}

impl Queryable for Source {
  async fn select_by_id(id: i32, conn: &mut sqlx::PgConnection) -> Result<Self, Error>
  where Self: Sized {
    Ok(sqlx::query_as::<_, Self>("
        SELECT sources.id,
               sources.url,
               sources.body,
               sources.params,
               sources.headers,
               sources.auth_type,
               sources.auth_username,
               sources.auth_password,
               sources.auth_token
        FROM sources
        WHERE sources.id = $1
      ")
      .bind(id)
      .fetch_one(conn)
      .await?)
  }

  async fn insert(&self, conn: &mut sqlx::PgConnection) -> Result<(), Error> {
    sqlx::query(&format!("
        INSERT INTO sources (
          url,
          body,
          params,
          headers,
          auth_type,
          auth_username,
          auth_password,
          auth_token
        )
        VALUES ($1, $2, $3, $4, $5, $6, $8)
      "))
      .bind(&self.url)
      .bind(&self.body)
      .bind(Json(&self.params))
      .bind(Json(&self.headers))
      .bind(&self.auth)
      .bind(self.auth.username())
      .bind(self.auth.password())
      .bind(self.auth.token())
      .execute(conn).await?;

    Ok(())
  }

  async fn update(&self, conn: &mut sqlx::PgConnection) -> Result<(), Error> {
    sqlx::query("
      UPDATE sources
      SET url = $1,
          body = $2,
          params = $3,
          headers = $4
          auth_type = $5,
          auth_username = $6,
          auth_password = $7,
          auth_token = $8
      WHERE sources.id = $10
    ")
    .bind(&self.url)
    .bind(&self.body)
    .bind(Json(&self.params))
    .bind(Json(&self.headers))
    .bind(&self.auth)
    .bind(self.auth.username())
    .bind(self.auth.password())
    .bind(self.auth.token())
    .bind(&self.id)
    .execute(conn)
    .await?;

    Ok(())
  }

  async fn delete(&self, conn: &mut sqlx::PgConnection) -> Result<(), Error> {
    sqlx::query("
      DELETE FROM sources
      WHERE sources.id = $1
    ")
    .bind(&self.id)
    .execute(conn)
    .await?;

    Ok(())
  }
}