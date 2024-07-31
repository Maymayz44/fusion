use api::FusionConfig;
use dotenv::dotenv;
use axum::{
  routing::get, Router
};
use std::error::Error;
use tokio::task;

pub mod api;
pub mod data;
pub mod config;
pub mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  dotenv()?;
  data::init_pool().await?;
  config::parse_config().await?;

  let fusion_server = task::spawn(async move {
    let fusion_config = FusionConfig::env();

    let fusion_router = Router::new()
      .route(format!("{}/*path", &fusion_config.path).as_str(), get(api::entrypoint));
    let fusion_listener = tokio::net::TcpListener::bind(fusion_config.full_address()).await.unwrap();
    
    println!("Fusion server running on http://{}:{}{}", &fusion_config.address, &fusion_config.port, &fusion_config.path);
    axum::serve(fusion_listener, fusion_router).await.unwrap();
  });

  fusion_server.await?;

  Ok(())
}