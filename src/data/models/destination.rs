use std::collections::HashMap;
use rocket::serde::{Deserialize, Serialize};
use sqlx::PgConnection;
use sqlx::{
  prelude::FromRow,
  types::Json,
};

use crate::data::Error;
use crate::data::Queryable;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Destination {
  pub id: Option<i32>,
  pub path: String,
  pub protocol: String,
  pub method: String,
  #[sqlx(json)]
  pub headers: HashMap<String, String>,
}

impl Destination {
  pub async fn select_by_path(path: String, conn: &mut PgConnection) -> Result<Self, Error> {
    Ok(sqlx::query_as::<_, Destination>(
      " SELECT destinations.id,
               destinations.path,
               destinations.protocol,
               destinations.method,
               destinations.headers
        FROM destinations
        WHERE destinations.path = $1 ")
      .bind(path)
      .fetch_one(conn)
      .await?)
  }
}

impl Queryable for Destination {
  async fn select_by_id(id: i32, conn: &mut PgConnection) -> Result<Self, Error>
  where Self: Sized {
    Ok(sqlx::query_as::<_, Destination>(
      " SELECT destinations.id,
               destinations.path,
               destinations.protocol,
               destinations.method,
               destinations.headers
        FROM destinations
        WHERE destinations.id = $1 ")
      .bind(id)
      .fetch_one(conn)
      .await?)
  }

  async fn insert(&self, conn: &mut PgConnection) -> Result<(), Error> {
    sqlx::query(
      " INSERT INTO destinations (path, protocol, method, headers)
        VALUES ($1, $2, $3, $4) ")
      .bind(&self.path)
      .bind(&self.protocol)
      .bind(&self.method)
      .bind(Json(&self.headers))
      .execute(conn)
      .await?;

    Ok(())
  }

  async fn update(&self, conn: &mut PgConnection) -> Result<(), Error> {
    sqlx::query(
      " UPDATE destinations
        SET protocol = $1,
            method = $2,
            headers = $3
        WHERE destinations.path = $4 ")
      .bind(&self.protocol)
      .bind(&self.method)
      .bind(Json(&self.headers))
      .bind(&self.path)
      .execute(conn)
      .await?;

    Ok(())
  }

  async fn delete(&self, conn: &mut PgConnection) -> Result<(), Error> {
    sqlx::query("DELETE FROM destinations WHERE path = $1")
      .bind(&self.path)
      .execute(conn)
      .await?;

    Ok(())
  }
}