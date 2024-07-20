use std::{sync::Arc, time::SystemTime};

use axum::extract::{FromRequestParts, Path, Request};
use reqwest::{header::HeaderMap, Client};
use serde_json::Value;
use regex::Regex;
use sqlx::PgConnection;
use tokio::{sync::RwLock, task::{self, JoinHandle}};

use crate::data::{models::{AuthToken, Destination, Source}, POOL};
pub use self::error::Error;
use self::response::Response;
pub use self::fusion_config::FusionConfig;

mod error;
mod response;
mod fusion_config;

pub async fn entrypoint(request: Request) -> Result<Response, Error> {
  let (request_parts, _) = request.into_parts();
  let path = format!("/{}", Path::<String>::from_request_parts(&mut request_parts.clone(), &()).await.unwrap().0);
  let mut conn = POOL.get()
    .ok_or_else(|| Error::InternalServerError(String::from("")))?
    .acquire().await?.detach();

  let destination = Destination::select_by_path(path, &mut conn).await?;

  authorize(&request_parts.headers, &destination, &mut conn).await?;

  let sources = fetch_sources(destination.get_sources(&mut conn).await?).await?;
  if let Some(filter) = destination.filter {
    let filtered_result = jq_rs::run(&filter, &sources.to_string())?.trim().to_string();
    return Ok(Response::JsonString(filtered_result));
  }

  Ok(Response::JsonString(sources.to_string()))
}

async fn fetch_sources(sources: Vec<Source>) -> Result<Value, Error> {
  let mut handles: Vec<JoinHandle<Result<Value, Error>>> = vec![];
  let timer = Arc::new(SystemTime::now());

  for source in sources {
    let timer = timer.clone();

    handles.push(task::spawn(async move {
      println!("Sending request for url: ({})", &source.url);

      let client = Client::new();
      let result = client
        .get(&source.url)
        .send().await?
        .json().await?;

      println!("Recieved response for url: ({}), took {} ms", &source.url, timer.elapsed().unwrap().as_millis());

      Ok(result)
    }));
  }

  let mut results = Vec::<Value>::new();
  for handle in handles {
    results.push(handle.await??);
  }

  Ok(Value::Array(results))
}

async fn authorize(headers: &HeaderMap, destination: &Destination, mut conn: &mut PgConnection) -> Result<(), Error> {
  let token = AuthToken::select_by_value(
    Regex::new(r"^Bearer\s\w{32}$").unwrap()
    .find_iter(headers
      .get("Authorization")
      .ok_or(Error::Unauthorized(()))?.to_str()?).next()
    .ok_or(Error::Unauthorized(()))?.as_str().split(' ').last()
    .ok_or(Error::Unauthorized(()))?.to_owned(), &mut conn).await?
    .ok_or(Error::Unauthorized(()))?;
  
  if !destination.is_token_for(&token, &mut conn).await? {
    return Err(Error::Unauthorized(()));
  }

  Ok(())
}