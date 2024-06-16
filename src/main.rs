use api::Bindings;
use dotenv::dotenv;
use axum::{
  routing::get, Router
};
use std::error::Error;

pub mod api;
pub mod data;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  dotenv()?;
  let bindings = Bindings::env();
  data::init_pool().await?;

  let app = Router::new()
    .route(format!("{}/*path", &bindings.path).as_str(), get(api::entrypoint));

  let listener = tokio::net::TcpListener::bind(bindings.full_address()).await.unwrap();
  axum::serve(listener, app).await?;

  Ok(())
}