use std::{collections::HashMap, str::FromStr};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{
  encode::IsNull, error::BoxDynError, postgres::{PgRow, PgTypeInfo}, types::Json, Database, Encode, FromRow, Postgres, Row, Type
};
use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;

use crate::config::{Error, YamlParser};

#[derive(Serialize, Deserialize, Debug)]
pub enum Body {
  None,
  Text(String),
  Json(Value),
  Form(HashMap<String, String>),
  Multi(HashMap<String, String>),
}

impl Body {
  pub fn text(&self) -> Option<String> {
    if let Self::Text(text) = self {
      return Some(text.clone());
    }
    None
  }

  pub fn json(&self) -> Option<Value> {
    if let Self::Json(json) = self {
      return Some(json.clone());
    }
    None
  }

  pub fn form(&self) -> Option<HashMap<String, String>> {
    if let Self::Form(form) = self {
      return Some(form.clone());
    }
    None
  }

  pub fn multi(&self) -> Option<HashMap<String, String>> {
    if let Self::Multi(multi) = self {
      return Some(multi.clone());
    }
    None
  }
}

impl ToString for Body {
  fn to_string(&self) -> String {
    String::from(match self {
      Self::None => "none",
      Self::Text(_) => "text",
      Self::Json(_) => "json",
      Self::Form(_) => "form",
      Self::Multi(_) => "multi",
    })
  }
}

impl Type<Postgres> for Body {
  fn type_info() -> PgTypeInfo {
    PgTypeInfo::with_name("body")
  }
}

impl Encode<'_, Postgres> for Body {
  fn encode_by_ref(&self, buf: &mut <Postgres as Database>::ArgumentBuffer<'_>) -> Result<IsNull, BoxDynError> {
    buf.extend(self.to_string().as_bytes());

    Ok(IsNull::No)
  }
}

impl FromRow<'_, PgRow> for Body {
  fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
    Ok(match row.try_get_unchecked("body_type")? {
      "text" => Self::Text(row.try_get("body_text")?),
      "json" => Self::Json(row.try_get("body_json")?),
      "form" => Self::Form(row.try_get::<Json<_>, _>("body_form")?.0),
      "multi" => Self::Multi(row.try_get::<Json<_>, _>("body_multi")?.0),
      "none" | _ => Self::None
    })
  }
}

impl TryFrom<Option<&YamlValue>> for Body {
  type Error = Error;

  fn try_from(value: Option<&YamlValue>) -> Result<Self, Self::Error> {
    Ok(if let Some(body) = value {
      match YamlParser::to_str_option(body.get("type"))? {
        Some("text") => Self::Text(YamlParser::to_string_req(body, "text")?),
        Some("json") => Self::Json(JsonValue::from_str(YamlParser::to_str(&YamlParser::get_req(body, "json")?)?)?),
        Some("form") => Self::Form(YamlParser::to_hashmap_option(body.get("form"))?.unwrap_or_default()),
        Some("multi") => Self::Multi(YamlParser::to_hashmap_option(body.get("form"))?.unwrap_or_default()),
        Some("none") | None => Self::None,
        Some(body_type) => Err(Error::String(format!("Souce body type `{}` invalid.", body_type)))?,
      }
    } else {
      Self::None
    })
  }
}