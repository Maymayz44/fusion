use crate::data::Error as DataError;
use sqlx::Error as SqlxError;
use reqwest::Error as ReqwestError;
use jq_rs::Error as JqError;
use serde_json::Error as JsonError;

#[derive(Responder)]
pub enum Error {
  #[response(status = 404)]
  NotFound(()),
  #[response(status = 400, content_type = "plain")]
  BadRequest(String),
  #[response(status = 401)]
  Unauthorized(()),
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

impl From<SqlxError> for Error {
  fn from(value: SqlxError) -> Self {
    Self::from(DataError::from(value))
  }
}

impl From<ReqwestError> for Error {
  fn from(value: ReqwestError) -> Self {
    Self::InternalServerError(value.to_string())
  }
}

impl From<JqError> for Error {
  fn from(value: JqError) -> Self {
    Self::InternalServerError(value.to_string())
  }
}

impl From<JsonError> for Error {
  fn from(value: JsonError) -> Self {
    Self::InternalServerError(value.to_string())
  }
}