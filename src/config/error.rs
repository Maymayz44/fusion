use chrono::ParseError as ChronoParseError;
use dotenv::Error as EnvError;
use std::io::Error as IoError;
use serde_yaml::Error as YamlError;
use crate::data::Error as DataError;
use sqlx::Error as SqlxError;

#[derive(Debug, thiserror::Error)]
pub enum Error<'a> {
  #[error("ENV ERROR: `{0}`")]
  Env(EnvError),
  #[error("FILE ERROR: `{0}`")]
  Io(IoError),
  #[error("YAML ERROR: {0}")]
  Yaml(YamlError),
  #[error("DATABASE ERROR: `{0}`")]
  Database(DataError),
  #[error("DATETIME PARSE ERROR: `{0}`")]
  Chrono(ChronoParseError),
  #[error("ERROR: `{0}`")]
  Str(&'a str),
}

impl From<EnvError> for Error<'_> {
  fn from(value: EnvError) -> Self {
    Self::Env(value)
  }
}

impl From<IoError> for Error<'_> {
  fn from(value: IoError) -> Self {
    Self::Io(value)
  }
}

impl From<YamlError> for Error<'_> {
  fn from(value: YamlError) -> Self {
    Self::Yaml(value)
  }
}

impl From<DataError> for Error<'_> {
  fn from(value: DataError) -> Self {
    Self::Database(value)
  }
}

impl From<SqlxError> for Error<'_> {
  fn from(value: SqlxError) -> Self {
    Self::from(DataError::from(value))
  }
}

impl From<ChronoParseError> for Error<'_> {
  fn from(value: ChronoParseError) -> Self {
    Self::Chrono(value)
  }
}