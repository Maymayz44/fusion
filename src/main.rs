#[macro_use]
extern crate rocket;

use dotenv::dotenv;

pub mod api;
pub mod data;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
  dotenv().ok();

  let _rocket = rocket::build()
  .mount("/api", routes![crate::api::entrypoint])
  .launch()
  .await?;

  Ok(())
}