use axum::{body::Body, response::{IntoResponse, Response as AxumResponse}};
use reqwest::StatusCode;

pub enum Response {
  JsonString(String)
}

impl IntoResponse for Response {
  fn into_response(self) -> AxumResponse {
    match self {
      Self::JsonString(json) => AxumResponse::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::new(json))
        .unwrap(),
    }
  }
}