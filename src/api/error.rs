use crate::data::Error as DataError;
use sqlx::Error as SqlxError;
use reqwest::Error as ReqwestError;

#[derive(Responder)]
pub enum Error {
  #[response(status = 404)]
  NotFound(()),
  #[response(status = 400, content_type = "plain")]
  BadRequest(String),
  #[response(status = 500, content_type = "plain")]
  InternalServerError(String),
}

impl From<DataError> for Error {
  fn from(value: DataError) -> Self {
    match value {
      DataError::Database(SqlxError::RowNotFound) => Self::NotFound(()),
      _ => Self::InternalServerError(value.to_string())
    }
  }
}

impl From<ReqwestError> for Error {
  fn from(value: ReqwestError) -> Self {
      Self::InternalServerError(value.to_string())
  }
}