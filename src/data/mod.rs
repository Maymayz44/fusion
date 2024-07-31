use std::env;
use std::sync::OnceLock;
use sqlx::{PgConnection, PgPool, Pool, Postgres};

pub use self::error::Error;
pub use self::queryable::{Queryable, QueryableCode};

mod error;
mod queryable;
pub mod models;
pub mod types;

static POOL: OnceLock<Pool<Postgres>> = OnceLock::new();

pub async fn init_pool() -> Result<(), Error> {
  if POOL.get().is_some() {
    return Err(Error::Str("Connection pool initialization failed."));
  }

  let database_url = env::var("DATABASE_URL")?;

  let pool = PgPool::connect(&database_url).await?;
  POOL.get_or_init(|| pool);

  Ok(())
}

pub async fn acquire_conn() -> Result<PgConnection, Error> {
  Ok(POOL.get()
    .ok_or(Error::Str("Connection could not be acquired from pool."))?
    .acquire().await?.detach())
}