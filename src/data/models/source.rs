use std::collections::HashMap;
use rocket::serde::{Deserialize, Serialize};
use sqlx::{
  prelude::FromRow,
  types::Json
};

use crate::data::{Error, Queryable};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Source {
  pub id: Option<i32>,
  pub url: String,
  pub body: String,
  #[sqlx(json)]
  pub params: HashMap<String, String>,
  #[sqlx(json)]
  pub headers: HashMap<String, String>,
}

impl Queryable for Source {
  async fn select_by_id(id: i32, conn: &mut sqlx::PgConnection) -> Result<Self, Error>
  where Self: Sized {
    Ok(sqlx::query_as::<_, Self>("
        SELECT sources.id,
               sources.url,
               sources.body,
               sources.params,
               sources.headers
        FROM sources
        WHERE sources.id = $1
      ")
      .bind(id)
      .fetch_one(conn)
      .await?)
  }

  async fn insert(&self, conn: &mut sqlx::PgConnection) -> Result<(), Error> {
    sqlx::query("
      INSERT INTO sources (url, body, params, headers)
      VALUES ($1, $2, $3, $4)
    ")
    .bind(&self.url)
    .bind(&self.body)
    .bind(Json(&self.params))
    .bind(Json(&self.headers))
    .execute(conn)
    .await?;

    Ok(())
  }

  async fn update(&self, conn: &mut sqlx::PgConnection) -> Result<(), Error> {
    sqlx::query("
      UPDATE sources
      SET url = $1,
          body = $2,
          params = $3,
          headers = $4
      WHERE sources.id = $5
    ")
    .bind(&self.url)
    .bind(&self.body)
    .bind(Json(&self.params))
    .bind(Json(&self.headers))
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