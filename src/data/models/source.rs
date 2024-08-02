use std::{collections::HashMap, time::Duration};
use serde::{Serialize, Deserialize};
use sqlx::{
  postgres::{types::PgInterval, PgRow}, prelude::FromRow, types::Json, PgConnection, Row
};

use crate::data::{queryable::QueryableCode, types::{Auth, Body}, Error, Queryable};

#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
  pub id: Option<i32>,
  pub code: String,
  pub url: String,
  pub params: HashMap<String, String>,
  pub headers: HashMap<String, String>,
  pub auth: Auth,
  pub timeout: Option<Duration>,
  pub body: Body,
}

impl Queryable for Source {
  async fn select_by_id(id: i32, conn: &mut PgConnection) -> Result<Self, Error> {
    Ok(sqlx::query_as::<_, Self>("
      SELECT sources.*
      FROM sources
      WHERE sources.id = $1;
    ")
    .bind(id)
    .fetch_one(conn)
    .await?)
  }

  async fn insert(&self, conn: &mut PgConnection) -> Result<Self, Error> {
    Ok(sqlx::query_as("
      INSERT INTO sources (
        code,
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
        body_json,
        body_form,
        body_multi
      )
      VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
      RETURNING sources.*;
    ")
    .bind(&self.code)
    .bind(&self.url)
    .bind(Json(&self.params))
    .bind(Json(&self.headers))
    .bind(&self.auth)
    .bind(self.auth.username())
    .bind(self.auth.password())
    .bind(self.auth.token())
    .bind(self.auth.param().map(|param| Json(param)))
    .bind(&self.timeout)
    .bind(&self.body)
    .bind(&self.body.text())
    .bind(&self.body.json())
    .bind(&self.body.form().map(|form| Json(form)))
    .bind(&self.body.multi().map(|multi| Json(multi)))
    .fetch_one(conn)
    .await?)
  }

  async fn update(&self, conn: &mut PgConnection) -> Result<Self, Error> {
    Ok(sqlx::query_as("
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
          body_json = $12,
          body_form = $13,
          body_multi = $14
      WHERE sources.code = $15
      RETURNING sources.*;
    ")
    .bind(&self.url)
    .bind(Json(&self.params))
    .bind(Json(&self.headers))
    .bind(&self.auth)
    .bind(self.auth.username())
    .bind(self.auth.password())
    .bind(self.auth.token())
    .bind(self.auth.param().map(|param| Json(param)))
    .bind(&self.timeout)
    .bind(&self.body)
    .bind(&self.body.text())
    .bind(&self.body.json())
    .bind(&self.body.form().map(|form| Json(form)))
    .bind(&self.body.multi().map(|multi| Json(multi)))
    .bind(&self.code)
    .fetch_one(conn)
    .await?)
  }

  async fn delete(&self, conn: &mut PgConnection) -> Result<(), Error> {
    sqlx::query("
      DELETE FROM sources
      WHERE sources.id = $1;
    ")
    .bind(&self.id)
    .execute(conn)
    .await?;

    Ok(())
  }

  async fn exists(&self, conn: &mut PgConnection) -> Result<bool, Error> {
    Ok(sqlx::query("SELECT EXISTS(
      SELECT *
      FROM sources
      WHERE sources.code = $1
    );")
    .bind(&self.code)
    .fetch_one(conn)
    .await?
    .try_get(0)?)
  }
}

impl QueryableCode for Source {
  async fn select_by_code(code: String, conn: &mut PgConnection) -> Result<Self, Error> {
    Ok(sqlx::query_as("
      SELECT *
      FROM sources
      WHERE sources.code = $1;
    ")
    .bind(code)
    .fetch_one(conn)
    .await?)
  }
}

impl FromRow<'_, PgRow> for Source {
  fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
    Ok(Self {
      id: row.try_get("id")?,
      code: row.try_get("code")?,
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