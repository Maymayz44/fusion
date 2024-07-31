use sqlx::PgConnection;
use super::Error;

#[allow(async_fn_in_trait)]
pub trait Queryable {
  async fn select_by_id(id: i32, conn: &mut PgConnection) -> Result<Self, Error>
  where Self: Sized;

  async fn insert(&self, conn: &mut PgConnection) -> Result<Self, Error>
  where Self: Sized;

  async fn update(&self, conn: &mut PgConnection) -> Result<Self, Error>
  where Self: Sized;

  async fn delete(&self, conn: &mut PgConnection) -> Result<(), Error>;

  async fn exists(&self, conn: &mut PgConnection) -> Result<bool, Error>;

  async fn insert_or_update(&self, conn: &mut PgConnection) -> Result<Self, Error>
  where Self: Sized {
    if self.exists(conn).await? {
      self.update(conn).await
    } else {
      self.insert(conn).await
    }
  }
}

#[allow(async_fn_in_trait)]
pub trait QueryableCode {
  async fn select_by_code(code: String, conn: &mut PgConnection) -> Result<Self, Error>
  where Self: Sized;
}