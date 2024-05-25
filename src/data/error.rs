use std::env::VarError;
use sqlx::Error as SqlxError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("An error happened while fetching environment variable: {0}")]
  Env(VarError),
  #[error("An error happened while contacting database: {0}")]
  Database(SqlxError),
  #[error("An error happened: {0}")]
  Other(Box<dyn std::error::Error + Send>),
  #[error("An error happened: {0}")]
  OtherString(String),
}

impl From<VarError> for Error {
  fn from(value: VarError) -> Self {
      Self::Env(value)
  }
}

impl From<SqlxError> for Error {
  fn from(value: SqlxError) -> Self {
      Self::Database(value)
  }
}