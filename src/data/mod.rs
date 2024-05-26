use std::env;
use std::sync::OnceLock;
use sqlx::{PgPool, Pool, Postgres};

mod error;
mod queryable;
pub mod models;

pub use self::error::Error;
pub use self::queryable::Queryable;

pub static POOL: OnceLock<Pool<Postgres>> = OnceLock::new();

pub async fn init_pool() -> Result<(), Error> {
  if POOL.get().is_some() {
    return Err(Error::ConnPoolInit(()));
  }

  let database_url = env::var("DATABASE_URL")?;

  let pool = PgPool::connect(&database_url).await?;
  POOL.get_or_init(|| pool);

  Ok(())
}