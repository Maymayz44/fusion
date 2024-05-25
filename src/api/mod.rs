use std::path::PathBuf;
use rocket::serde::json::Value;

use crate::data::conn;
use self::error::Error;

mod error;

#[get("/<path..>")]
pub async fn entrypoint(path: PathBuf) -> Result<Value, Error> {
  let mut conn = conn().await?;

  let fullpath = String::from("/") + &path
    .into_os_string().into_string()
    .map_err(|_|
      Error::InternalServerError(String::from("Could not convert the request path to type `String`")))?
    .replace("\\", "/");

  Ok(Value::Null)
}