use std::{path::PathBuf, str::FromStr};
use rocket::serde::json::Value;
use reqwest::Client;

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

  let destination = Destination::select_by_path(fullpath, &mut conn).await?;
  let sources = Value::Array(fetch_sources(destination.get_sources(&mut conn).await?).await?);

  if let Some(filter) = destination.filter {
    let filtered_result = jq_rs::run(&filter, &sources.to_string())?.trim().to_string();
    return Ok(Value::from_str(&filtered_result)?);
  }

  Ok(sources)
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