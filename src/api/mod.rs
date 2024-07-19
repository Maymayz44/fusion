use std::{collections::HashMap, sync::Arc};

use axum::extract::{FromRequestParts, Path, Request};
use reqwest::{header::{HeaderMap, HeaderValue}, Client};
use serde_json::Value;
use regex::Regex;
use sqlx::PgConnection;
use tokio::{sync::RwLock, task::{self, JoinHandle}};

use crate::data::{models::{AuthToken, Destination, Source}, types::Auth, POOL};
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

// async fn fetch_sources(sources: Vec<Source>) -> Result<Value, Error> {
//   let client = Client::new();
  
//   let mut result = Vec::<Value>::new();
//   for source in sources {
//     let mut request = client
//       .get(&source.url)
//       .query(&source.params);

//     if let Ok(headers) = HeaderMap::<HeaderValue>::try_from(&source.headers) {
//       request = request.headers(headers);
//     }

//     match source.auth {
//       Auth::Basic { username, password } => {
//         request = request.basic_auth(username, Some(password));
//       },
//       Auth::Bearer { token } => {
//         request = request.bearer_auth(token);
//       },
//       Auth::None => (),
//     }

//     result.push(request
//       .send()
//       .await?
//       .json()
//       .await?);
//   }

//   Ok(Value::Array(result))
// }

async fn fetch_sources(sources: Vec<Source>) -> Result<Value, Error> {
  let results_ref = Arc::new(RwLock::new(Vec::<(u16, Value)>::new()));

  let mut handles: Vec<JoinHandle<()>> = vec![];
  let mut i = 0;
  for source in sources {
    let j = i.clone();
    i += 1;
    
    let results = Arc::clone(&results_ref);
    handles.push(task::spawn(async move {
      let client = Client::new();

      let result = (j, client.get(&source.url)
        .send()
        .await.unwrap()
        .json()
        .await.unwrap());

      println!("{}", j);
      results.write().await.push(result);
    }));
  }

  for handle in handles {
    handle.await?;
  }

  let mut results = results_ref.clone().read().await.to_vec(); 
  results.sort_by(|a, b| a.0.cmp(&b.0));
  Ok(Value::Array(results.iter().map(|result| result.clone().1).collect()))
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