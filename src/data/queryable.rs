use sqlx::PgConnection;
use super::Error;

#[allow(async_fn_in_trait)]
pub trait Queryable {
  async fn select_by_id(id: i32, conn: &mut PgConnection) -> Result<Self, Error>
  where Self: Sized;

  async fn insert(&self, conn: &mut PgConnection) -> Result<(), Error>;

  async fn update(&self, conn: &mut PgConnection) -> Result<(), Error>;

  async fn delete(&self, conn: &mut PgConnection) -> Result<(), Error>;
}