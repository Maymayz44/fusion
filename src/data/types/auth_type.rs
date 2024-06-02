use rocket::serde::{Deserialize, Serialize};
use sqlx::{encode::IsNull, postgres::PgTypeInfo, Encode, Postgres, Type};

#[derive(Serialize, Deserialize)]
pub enum AuthType {
  None,
  Basic { username: String, password: String },
  Bearer { token: String },
}

impl ToString for AuthType {
  fn to_string(&self) -> String {
    match self {
      Self::None => "none".to_owned(),
      Self::Basic { username: _, password: _ } => "basic".to_owned(),
      Self::Bearer { token: _  } => "bearer".to_owned(),
    }
  }
}

impl Type<Postgres> for AuthType {
  fn type_info() -> PgTypeInfo {
    PgTypeInfo::with_name("auth_type")
  }
}

impl Encode<'_, Postgres> for AuthType {
  fn encode_by_ref(&self, buf: &mut <Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer) -> IsNull {
    buf.extend(self.to_string().as_bytes());

    IsNull::No
  }
}

impl AuthType {
  pub fn username(&self) -> Option<String> {
    if let Self::Basic { username, password: _ } = self {
      return Some(username.to_string());
    }
    None
  }

  pub fn password(&self) -> Option<String> {
    if let Self::Basic { username: _, password } = self {
      return Some(password.to_string());
    }
    None
  }

  pub fn token(&self) -> Option<String> {
    if let Self::Bearer { token } = self {
      return Some(token.to_string());
    }
    None
  }
}