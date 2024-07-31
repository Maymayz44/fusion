use chrono::ParseError as ChronoParseError;
use dotenv::Error as EnvError;
use std::io::Error as IoError;
use serde_yaml::Error as YamlError;
use crate::data::Error as DataError;
use sqlx::Error as SqlxError;
use serde_json::Error as JsonError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("ENV ERROR: `{0}`")]
  Env(EnvError),
  #[error("FILE ERROR: `{0}`")]
  Io(IoError),
  #[error("YAML ERROR: `{0}`")]
  Yaml(YamlError),
  #[error("JSON ERROR: `{0}`")]
  Json(JsonError),
  #[error("DATABASE ERROR: `{0}`")]
  Database(DataError),
  #[error("DATETIME PARSE ERROR: `{0}`")]
  Chrono(ChronoParseError),
  #[error("ERROR: `{0}`")]
  Str(&'static str),
  #[error("ERROR: `{0}`")]
  String(String),
}

impl From<EnvError> for Error {
  fn from(value: EnvError) -> Self {
    Self::Env(value)
  }
}

impl From<IoError> for Error {
  fn from(value: IoError) -> Self {
    Self::Io(value)
  }
}

impl From<YamlError> for Error {
  fn from(value: YamlError) -> Self {
    Self::Yaml(value)
  }
}

impl From<DataError> for Error {
  fn from(value: DataError) -> Self {
    Self::Database(value)
  }
}

impl From<SqlxError> for Error {
  fn from(value: SqlxError) -> Self {
    Self::from(DataError::from(value))
  }
}

impl From<ChronoParseError> for Error {
  fn from(value: ChronoParseError) -> Self {
    Self::Chrono(value)
  }
}

impl From<JsonError> for Error {
  fn from(value: JsonError) -> Self {
    Self::Json(value)
  }
}