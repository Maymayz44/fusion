use std::env;
use sqlx::{postgres::PgConnection, Connection};

mod error;
mod queryable;
pub mod models;

pub use self::error::Error;
pub use self::queryable::Queryable;

pub async fn conn() -> Result<PgConnection, Error> {
  let database_url = env::var("DATABASE_URL")?;

  let conn = PgConnection::connect(&database_url).await?;

  Ok(conn)
}