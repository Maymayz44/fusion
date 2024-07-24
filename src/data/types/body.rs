use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{
  encode::IsNull, error::BoxDynError,
  postgres::{PgRow, PgTypeInfo},
  Database, Encode, FromRow,
  Postgres, Type, Row
};

#[derive(Serialize, Deserialize)]
pub enum Body {
  None,
  Text(String),
  Json(Value)
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
}

impl ToString for Body {
  fn to_string(&self) -> String {
    String::from(match self {
      Self::None => "none",
      Self::Text(_) => "text",
      Self::Json(_) => "json",
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
      "none" | _ => Self::None
    })
  }
}