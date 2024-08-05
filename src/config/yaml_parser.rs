use std::{
  collections::HashMap, ops::Deref, str::FromStr, time::Duration
};
use chrono::{DateTime, Utc};
use serde_yaml::Value as YamlValue;
use serde_json::Value as JsonValue;

use super::Error;

pub struct YamlParser;

impl YamlParser {
  pub fn get_req(data: &YamlValue, value: &str) -> Result<YamlValue, Error> {
    data.get(value).map(|val| val.clone())
      .ok_or(Error::String(format!("Required field `{}` is missing.", value)))
  }

  pub fn to_hashmap_option(mapping: Option<&YamlValue>) -> Result<Option<HashMap<String, String>>, Error> {
    mapping.map(|val|
      val.as_mapping()
      .ok_or(Error::Str("`Value` could not be converted to `Mapping`."))?
      .iter()
      .map(|(key, value)|
        Ok((Self::to_string(key)?, serde_yaml::to_string(value)?.replace('\n', ""))))
      .collect()
    )
    .transpose()
  }

  pub fn to_str(value: &YamlValue) -> Result<&str, Error> {
    value.as_str().ok_or(Error::Str("`Value` could not be converted to `str`."))
  }

  pub fn to_string(value: &YamlValue) -> Result<String, Error> {
    value.as_str().map(|str| str.to_owned())
      .ok_or(Error::Str("`Value` could not be converted to `String`."))
  }

  pub fn to_u64(value: &YamlValue) -> Result<u64, Error> {
    value.as_u64().ok_or(Error::Str("`Value` could not be converted to `u64`."))
  }

  pub fn to_duration(value: Option<&YamlValue>) -> Result<Option<Duration>, Error> {
    value.map(|val| Ok(Duration::from_secs(Self::to_u64(val)?))).transpose()
  }

  pub fn to_string_option(value: Option<&YamlValue>) -> Result<Option<String>, Error> {
    value.map(|val| Self::to_string(val)).transpose()
  }

  pub fn to_str_option(value: Option<&YamlValue>) -> Result<Option<&str>, Error> {
    value.map(|val| Self::to_str(val)).transpose()
  }

  pub fn to_bool_option(value: Option<&YamlValue>) -> Result<Option<bool>, Error> {
    value.map(|val| Ok(val.as_bool().ok_or(Error::Str("`Value` could not be converted to `bool`."))?)).transpose()
  }

  pub fn to_datetime_option(value: Option<&YamlValue>) -> Result<Option<DateTime<Utc>>, Error> {
    value.map(|val| Ok(DateTime::from_str(Self::to_str(val)?)?)).transpose()
  }

  pub fn to_string_req(data: &YamlValue, value: &str) -> Result<String, Error> {
    Self::to_string(&Self::get_req(data, value)?)
  }

  pub fn to_string_option_multiline(value: Option<&YamlValue>) -> Result<Option<String>, Error> {
    Ok(Self::to_str_option(value)?.map(|val| val.replace('\n', "").trim()
      .split_whitespace().collect::<Vec<_>>().join(" ")))
  }

  pub fn vec_to_string(value: &Vec<YamlValue>) -> Result<Vec<String>, Error> {
    value.iter().map(|val| Self::to_string(val)).collect()
  }

  pub fn to_json_option(value: Option<&YamlValue>) -> Result<Option<JsonValue>, Error> {
    Self::to_string_option_multiline(value)?
      .map(|val| Ok(JsonValue::from_str(val.as_str())?)).transpose()
  }
}