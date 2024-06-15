use std::{path::PathBuf, str::FromStr};
use rocket::serde::json::Value;
use reqwest::{header::{HeaderMap, HeaderValue}, Client};

use crate::data::{models::{AuthToken, Destination, Source}, types::Auth, POOL};
pub use self::error::Error;
use self::response::Response;

mod error;
mod response;

#[get("/<path..>")]
pub async fn entrypoint(path: PathBuf, auth: AuthToken) -> Result<Response, Error> {
  let mut conn: sqlx::pool::PoolConnection<sqlx::Postgres> = POOL.get()
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
    return Ok(Response::JsonString(filtered_result));
  }

  Ok(Response::JsonString(sources.to_string()))
}

async fn fetch_sources(sources: Vec<Source>) -> Result<Value, Error> {
  let client = Client::new();
  
  let mut result = Vec::<Value>::new();
  for source in sources {
    let mut request = client
      .get(&source.url)
      .query(&source.params);

    if let Ok(headers) = HeaderMap::<HeaderValue>::try_from(&source.headers) {
      request = request.headers(headers);
    }

    match source.auth {
      Auth::Basic { username, password } => {
        request = request.basic_auth(username, Some(password));
      },
      Auth::Bearer { token } => {
        request = request.bearer_auth(token);
      },
      Auth::None => (),
    }

    result.push(request
      .send()
      .await?
      .json()
      .await?);
  }

  Ok(Value::Array(result))
}