use std::{path::PathBuf, str::FromStr};
use rocket::serde::json::Value;
use reqwest::Client;

use crate::data::{models::{AuthType, Destination, Source}, POOL};
use self::error::Error;

mod error;

#[get("/<path..>")]
pub async fn entrypoint(path: PathBuf) -> Result<Value, Error> {
  let mut conn = POOL.get()
    .ok_or_else(|| Error::InternalServerError(String::from("")))?
    .acquire().await?;

  let fullpath = String::from("/") + &path
    .into_os_string().into_string()
    .map_err(|_|
      Error::InternalServerError(String::from("Could not convert the request path to type `String`")))?
    .replace("\\", "/");

  let destination = Destination::select_by_path(fullpath, &mut conn).await?;
  let sources = fetch_sources(destination.get_sources(&mut conn).await?).await?;

  if let Some(filter) = destination.filter {
    let filtered_result = jq_rs::run(&filter, &sources.to_string())?.trim().to_string();
    return Ok(Value::from_str(&filtered_result)?);
  }

  Ok(sources)
}

async fn fetch_sources(sources: Vec<Source>) -> Result<Value, Error> {
  let client = Client::new();
  
  let mut result = Vec::<Value>::new();
  for source in sources {
    let mut request = client
      .get(&source.url)
      .query(&source.params);

    match source.auth {
      AuthType::Basic { username, password } => {
        request = request.basic_auth(username, Some(password));
      },
      AuthType::Bearer { token } => {
        request = request.bearer_auth(token);
      },
      AuthType::None => {},
    }

    result.push(request
      .send()
      .await?
      .json()
      .await?);
  }

  Ok(Value::Array(result))
}