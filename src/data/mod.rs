use std::env;
use std::sync::OnceLock;
use sqlx::{PgConnection, PgPool, Pool, Postgres};

mod error;
mod queryable;
pub mod models;
pub mod types;

pub use self::error::Error;
pub use self::queryable::Queryable;

static POOL: OnceLock<Pool<Postgres>> = OnceLock::new();

pub async fn init_pool() -> Result<(), Error> {
  if POOL.get().is_some() {
    return Err(Error::Connection(String::from("Connection pool initialization failed")));
  }

  let database_url = env::var("DATABASE_URL")?;

  let pool = PgPool::connect(&database_url).await?;
  POOL.get_or_init(|| pool);

  Ok(())
}

pub async fn acquire_conn() -> Result<PgConnection, Error> {
  Ok(POOL.get()
    .ok_or_else(|| Error::Connection(String::from("Connection could not be acquired from pool")))?
    .acquire().await?.detach())
}