#[macro_use]
extern crate rocket;

use dotenv::dotenv;

pub mod api;
pub mod data;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
  dotenv().ok();

  data::init_pool().await
    .map_err(|err| panic!("Database connection pool could not be initialized due to the following error: {}", err)).unwrap();

  let _rocket = rocket::build()
  .mount("/api", routes![crate::api::entrypoint])
  .launch()
  .await?;

  Ok(())
}