use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_json::Value;
use sqlx::{encode::IsNull, postgres::{PgRow, PgTypeInfo}, types::Json, Encode, FromRow, Postgres, Row, Type};
use crate::data::Error;

#[derive(Serialize, Deserialize)]
pub enum Auth {
  None,
  Basic { username: String, password: String },
  Bearer { token: String },
  Param(String, String)
}

impl Auth {
  pub fn username(&self) -> Option<String> {
    if let Self::Basic { username, password: _ } = self {
      return Some(username.clone());
    }
    None
  }

  pub fn password(&self) -> Option<String> {
    if let Self::Basic { username: _, password } = self {
      return Some(password.clone());
    }
    None
  }

  pub fn token(&self) -> Option<String> {
    if let Self::Bearer { token } = self {
      return Some(token.clone());
    }
    None
  }

  pub fn param(&self) -> Option<(String, String)> {
    if let Self::Param(key, value) = self {
      return Some((key.clone(), value.clone()));
    }
    None
  }
}

impl ToString for Auth {
  fn to_string(&self) -> String {
    match self {
      Self::None => String::from("none"),
      Self::Basic { username: _, password: _ } => String::from("basic"),
      Self::Bearer { token: _ } => String::from("bearer"),
      Self::Param(_, _) => String::from("param")
    }
  }
}

impl Type<Postgres> for Auth {
  fn type_info() -> PgTypeInfo {
    PgTypeInfo::with_name("auth_type")
  }
}

impl Encode<'_, Postgres> for Auth {
  fn encode_by_ref(&self, buf: &mut <Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer) -> IsNull {
    buf.extend(self.to_string().as_bytes());

    IsNull::No
  }
}

impl FromRow<'_, PgRow> for Auth {
  fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
    Ok(match row.try_get_unchecked("auth_type")? {
      "basic" => Self::Basic { username: row.try_get("auth_username")?, password: row.try_get("auth_password")? },
      "bearer" => Self::Bearer { token: row.try_get("auth_token")? },
      "param" => {
        let json: Json<HashMap<String, String>> = row.try_get("auth_param")?;
        let vals: (&String, &String) = json.0.iter().next().unwrap();

        Self::Param(vals.0.to_owned(), vals.1.to_owned())
      },
      "none" | _ => Self::None,
    })
  }
}