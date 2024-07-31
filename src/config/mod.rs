use std::{fs::File, io::Read};
use chrono::Utc;
use serde_yaml::Value as YamlValue;
use sqlx::{PgConnection, Row};

use crate::{
  data::{
    acquire_conn,
    models::{
      AuthToken,
      Destination,
      Source
    },
    Queryable
  },
  utils::hash::Hasher
};
pub use self::error::Error;
pub use self::yaml_parser::YamlParser;

mod error;
mod yaml_parser;

pub async fn parse_config() -> Result<(), Error> {
  let mut config_file = File::open(dotenv::var("CONFIG_FILE")
    .unwrap_or_else(|_| String::from("/etc/fusion/fusion.yaml")))?;
  let mut config_string = String::new();
  config_file.read_to_string(&mut config_string)?;
  let config = serde_yaml::from_str::<YamlValue>(&config_string)?;

  let mut conn = acquire_conn().await?;

  let prev_config_ver = sqlx::query("
      SELECT config_versions.hash
      FROM config_versions
      ORDER BY config_versions.updated_on DESC
      LIMIT 1;
    ")
    .fetch_optional(&mut conn)
    .await?;

  let result = &Hasher::hash_string(serde_yaml::to_string(&config)?)[..];

  match prev_config_ver {
    Some(row) => {
      if row.try_get::<&[u8], _>("hash")? != result {
        println!("Configuration changed, updating database.");
        update_config(&mut conn, config, result).await?;
      }
    },
    None => {
      println!("No previous configuration found, initializing database.");
      update_config(&mut conn, config, result).await?;
    },
  }

  Ok(())
}

async fn update_config(conn: &mut PgConnection, config: YamlValue, hash: &[u8]) -> Result<(), Error> {
  let config = config.as_mapping().ok_or(Error::Str(""))?;

  if let Some(YamlValue::Mapping(sources)) = config.get("sources") {
    for (code, data) in sources {
      Source {
        id: None,
        code: YamlParser::to_string(code)?,
        url: YamlParser::to_string_req(data, "url")?,
        params: YamlParser::to_hashmap_option(data.get("params"))?.unwrap_or_default(),
        headers: YamlParser::to_hashmap_option(data.get("headers"))?.unwrap_or_default(),
        timeout: YamlParser::to_duration(data.get("timeout"))?,
        auth: data.get("auth").try_into()?,
        body: data.get("body").try_into()?,
      }
      .insert_or_update(conn).await?;
    }
  }

  if let Some(YamlValue::Mapping(destinations)) = config.get("destinations") {
    for (code, data) in destinations {
      let dest = Destination {
        id: None,
        code: YamlParser::to_string(code)?,
        path: YamlParser::to_string_req(data, "path")?,
        headers: YamlParser::to_hashmap_option(data.get("headers"))?.unwrap_or_default(),
        is_auth: YamlParser::to_bool_option(data.get("is_auth"))?.unwrap_or_default(),
        filter: YamlParser::to_string_option_multiline(data.get("filter"))?,
      }
      .insert_or_update(conn).await?;

      if let Some(YamlValue::Sequence(dest_sources)) = data.get("sources") {
        dest.unlink_sources(conn).await?;
        dest.link_sources(YamlParser::vec_to_string(dest_sources)?, conn).await?;
      }
    }
  }

  if let Some(YamlValue::Sequence(auth_tokens)) = config.get("auth_tokens") {
    for value in auth_tokens {
      let token = match value {
        YamlValue::String(val) => AuthToken {
          id: None,
          value: val.to_owned(),
          expiration: None,
        },
        YamlValue::Mapping(_) => AuthToken {
          id: None,
          value: YamlParser::to_string_req(value, "value")?,
          expiration: YamlParser::to_datetime_option(value.get("expiration"))?,
        },
        _ => Err(Error::Str("`Value` could not be converted to `AuthToken`"))?
      }
      .insert_or_update(conn).await?;
      
      if let Some(YamlValue::Sequence(token_dests)) = value.get("destinations") {
        token.unlink_destinations(conn).await?;
        token.link_destinations(YamlParser::vec_to_string(token_dests)?, conn).await?;
      }
    }
  }

  sqlx::query("
    INSERT INTO config_versions (updated_on, hash)
    VALUES ($1, $2)
  ")
  .bind(Utc::now())
  .bind(hash)
  .execute(conn)
  .await?;

  Ok(())
}