use std::env::VarError;
use sqlx::Error as SqlxError;
use std::io::Error as StdError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("ENV ERROR: `{0}`")]
  Env(VarError),
  #[error("DATABASE ERROR: `{0}`")]
  Database(SqlxError),
  #[error("DATABASE CONNECTION ERROR: `{0}`")]
  Connection(String),
  #[error("DATA ERROR: `{0}`")]
  Str(&'static str),
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

impl Into<StdError> for Error {
  fn into(self) -> StdError {
    StdError::other(self.to_string())
  }
}