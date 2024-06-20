use crate::data::Error as DataError;
use axum::response::IntoResponse;
use http::header::ToStrError;
use sqlx::Error as SqlxError;
use reqwest::{Error as ReqwestError, StatusCode};
use jq_rs::Error as JqError;
use serde_json::Error as JsonError;

#[derive(Debug)]
pub enum Error {
  NotFound(()),
  BadRequest(String),
  Unauthorized(()),
  InternalServerError(String),
}

impl IntoResponse for Error {
  fn into_response(self) -> axum::response::Response {
    match self {
      Self::NotFound(()) => (StatusCode::NOT_FOUND, String::new()),
      Self::BadRequest(err) => (StatusCode::BAD_REQUEST, err),
      Self::Unauthorized(()) => (StatusCode::UNAUTHORIZED, String::new()),
      Self::InternalServerError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err),
    }.into_response()
  }
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

impl From<ToStrError> for Error {
  fn from(value: ToStrError) -> Self {
    Self::InternalServerError(value.to_string())
  }
}