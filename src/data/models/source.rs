use std::{collections::HashMap, time::Duration};
use serde::{Serialize, Deserialize};
use sqlx::{
  postgres::{types::PgInterval, PgRow}, prelude::FromRow, types::Json, Row
};

use crate::data::{types::{Auth, Body}, Error, Queryable};

#[derive(Serialize, Deserialize)]
pub struct Source {
  pub id: Option<i32>,
  pub url: String,
  pub params: HashMap<String, String>,
  pub headers: HashMap<String, String>,
  pub auth: Auth,
  pub timeout: Option<Duration>,
  pub body: Body,
}

impl FromRow<'_, PgRow> for Source {
  fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
    Ok(Self {
      id: row.try_get("id")?,
      url: row.try_get("url")?,
      params: row.try_get::<Json<HashMap<String, String>>, _>("params")?.0,
      headers: row.try_get::<Json<HashMap<String, String>>, _>("headers")?.0,
      auth: Auth::from_row(row)?,
      timeout: row.try_get::<Option<PgInterval>, _>("timeout")?
        .map(|interval| Duration::from_micros(interval.microseconds as u64)),
      body: Body::from_row(row)?,
    })
  }
}

impl Queryable for Source {
  async fn select_by_id(id: i32, conn: &mut sqlx::PgConnection) -> Result<Self, Error>
  where Self: Sized {
    Ok(sqlx::query_as::<_, Self>("
        SELECT sources.id,
               sources.url,
               sources.params,
               sources.headers,
               sources.auth_type,
               sources.auth_username,
               sources.auth_password,
               sources.auth_token,
               sources.auth_param,
               sources.timeout,
               sources.body_type,
               sources.body_text,
               sources.body_json
        FROM sources
        WHERE sources.id = $1;
      ")
      .bind(id)
      .fetch_one(conn)
      .await?)
  }

  async fn insert(&self, conn: &mut sqlx::PgConnection) -> Result<(), Error> {
    sqlx::query(&format!("
        INSERT INTO sources (
          url,
          params,
          headers,
          auth_type,
          auth_username,
          auth_password,
          auth_token,
          auth_param,
          timeout,
          body_type,
          body_text,
          body_json
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12);
      "))
      .bind(&self.url)
      .bind(&self.body)
      .bind(Json(&self.params))
      .bind(Json(&self.headers))
      .bind(&self.auth)
      .bind(self.auth.username())
      .bind(self.auth.password())
      .bind(self.auth.token())
      .bind(Json(self.auth.param()))
      .bind(&self.timeout)
      .execute(conn).await?;

    Ok(())
  }

  async fn update(&self, conn: &mut sqlx::PgConnection) -> Result<(), Error> {
    sqlx::query("
      UPDATE sources
      SET url = $1,
          params = $2,
          headers = $3,
          auth_type = $4,
          auth_username = $5,
          auth_password = $6,
          auth_token = $7,
          auth_param = $8,
          timeout = $9,
          body_type = $10,
          body_text = $11,
          body_json = $12
      WHERE sources.id = $13;
    ")
    .bind(&self.url)
    .bind(Json(&self.params))
    .bind(Json(&self.headers))
    .bind(&self.auth)
    .bind(self.auth.username())
    .bind(self.auth.password())
    .bind(self.auth.token())
    .bind(Json(self.auth.param()))
    .bind(&self.timeout)
    .bind(&self.body)
    .bind(&self.body.text())
    .bind(&self.body.json())
    .bind(&self.id)
    .execute(conn)
    .await?;

    Ok(())
  }

  async fn delete(&self, conn: &mut sqlx::PgConnection) -> Result<(), Error> {
    sqlx::query("
      DELETE FROM sources
      WHERE sources.id = $1;
    ")
    .bind(&self.id)
    .execute(conn)
    .await?;

    Ok(())
  }
}