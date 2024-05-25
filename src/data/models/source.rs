use std::collections::HashMap;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Source {
  pub id: i32,
  pub url: String,
  pub body: String,
  pub params: HashMap<String, String>,
}