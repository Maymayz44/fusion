
#[derive(Responder)]
pub enum Response {
  #[response(status = 200, content_type = "json")]
  Json(String)
}