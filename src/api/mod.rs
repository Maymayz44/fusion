use std::path::PathBuf;
use rocket::serde::json::{Value, json};
use reqwest::Client;
use serde_json::Map;

use crate::data::{conn, models::{Destination, Source}};
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

  let dest = Destination::select_by_path(fullpath, &mut conn).await?;
  let sources = dest.get_sources(&mut conn).await?;

  Ok(Value::Array(fetch_sources(sources).await?))
}

async fn fetch_sources(sources: Vec<Source>) -> Result<Vec<Value>, Error> {
  let client = Client::new();
  let mut result = Vec::<Value>::new();

  for source in sources {
    result.push(client.get(&source.url)
      .send()
      .await?
      .json()
      .await?);
  }

  Ok(result)
}