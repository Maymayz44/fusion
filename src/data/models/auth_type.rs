use rocket::serde::{Deserialize, Serialize};

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