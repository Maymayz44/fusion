use std::{collections::HashMap, fs::File, io::Read, str::FromStr, time::Duration};
use chrono::{DateTime, Utc};
use serde_yaml::Value;
use sha2::{Digest, Sha256};
use sqlx::{PgConnection, Row};

use crate::data::{acquire_conn, models::{AuthToken, Destination, Source}, types::{Auth, Body}, Queryable};
use error::Error;

mod error;

pub async fn parse_config<'a>() -> Result<(), Error<'a>> {
  let mut config_file = File::open(dotenv::var("CONFIG_FILE")
    .unwrap_or_else(|_| String::from("/etc/fusion/fusion.yaml")))?;
  let mut config_string = String::new();
  config_file.read_to_string(&mut config_string)?;
  let config = serde_yaml::from_str::<Value>(&config_string)?;

  let mut conn = acquire_conn().await?;

  let prev_config_ver = sqlx::query("
      SELECT config_versions.hash
      FROM config_versions
      ORDER BY config_versions.updated_on DESC
      LIMIT 1;
    ")
    .fetch_optional(&mut conn)
    .await?;

  let mut hasher = Sha256::new();
  hasher.update(serde_yaml::to_string(&config).unwrap());
  let result = &hasher.finalize()[..];

  match prev_config_ver {
    Some(row) => {
      if row.try_get::<&[u8], _>("hash")? != result {
        println!("Configuration changed, updating data.");
        update_config(&mut conn, config, result).await?;
      }
    },
    None => {
      println!("No previous configuration found, initializing data.");
      update_config(&mut conn, config, result).await?;
    },
  }

  // println!("{:x?}", result);
  // println!("{}", serde_yaml::to_string(&config).unwrap());

  Ok(())
}

async fn update_config<'a>(conn: &mut PgConnection, config: Value, hash: &[u8]) -> Result<(), Error<'a>> {
  if let Value::Mapping(mapping) = config {
    if let Some(Value::Mapping(sources)) = mapping.get("sources") {
      for (code, data) in sources {
        Source {
          id: None,
          code: to_string(Some(code)),
          url: data.get("url").map(|data| data.as_str().unwrap().to_owned())
            .ok_or(Error::Str("url is required for sources"))?,
          params: to_hashmap(data.get("params"))?,
          headers: to_hashmap(data.get("headers"))?,
          timeout: data.get("timeout").map(|data| Duration::from_secs(data.as_u64().unwrap())),
          auth: data.get("auth").map(|data| match to_str(data.get("type")) {
            "basic" => Auth::Basic { username: to_string(data.get("username")), password: to_string(data.get("password")) },
            "bearer" => Auth::Bearer { token: to_string(data.get("token")) },
            "param" => Auth::Param(to_string(data.get("key")), to_string(data.get("value"))),
            "none" | _ => Auth::None,
          }).unwrap_or(Auth::None),
          body: data.get("body").map(|data| match to_str(data.get("type")) {
            "none" | _ => Body::None
          }).unwrap_or(Body::None),
        }.insert_or_update(conn).await?;
      }
    }

    if let Some(Value::Mapping(destinations)) = mapping.get("destinations") {
      for (code, data) in destinations {
        let dest = Destination {
          id: None,
          code: to_string(Some(code)),
          path: data.get("path").map(|data| data.as_str().unwrap().to_owned())
            .ok_or(Error::Str("path is required for destinations"))?,
          headers: to_hashmap(data.get("headers"))?,
          is_auth: data.get("is_auth").map(|value| value.as_bool().unwrap()).unwrap_or(false),
          filter: data.get("filter").map(|value|
            value.as_str().unwrap().replace('\n', "").trim()
            .split_whitespace().collect::<Vec<_>>().join(" ").to_owned()),
        }.insert_or_update(conn).await?;

        if let Some(Value::Sequence(dest_sources)) = data.get("sources") {
          dest.unlink_sources(conn).await?;
          dest.link_sources(dest_sources.iter().map(|value| to_string(Some(value))).collect(), conn).await?;
        }
      }
    }

    if let Some(Value::Sequence(auth_tokens)) = mapping.get("auth_tokens") {
      for token in auth_tokens {
        if let Value::Mapping(data) = token {
          let token = AuthToken {
            id: None,
            value: to_string(data.get("value")),
            expiration: data.get("expiration").map(|data| DateTime::from_str(to_str(Some(data))).unwrap()),
          }.insert_or_update(conn).await?;
        
          if let Some(Value::Sequence(token_dests)) = data.get("destinations") {
            token.unlink_destinations(conn).await?;
            token.link_destinations(token_dests.iter().map(|value| to_string(Some(value))).collect(), conn).await?;
          }
        }
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

fn to_str(value: Option<&Value>) -> &str {
  value.map(|data| data.as_str().unwrap()).unwrap()
}

fn to_string(value: Option<&Value>) -> String {
  value.map(|data| data.as_str().unwrap()).unwrap().to_owned()
}

fn to_hashmap<'a>(mapping: Option<&Value>) -> Result<HashMap<String, String>, Error<'a>> {
  Ok(mapping.map(|data|
    data.as_mapping().unwrap().iter()
    .map(|(key, value)|
      (key.as_str().unwrap().to_owned(),
      serde_yaml::to_string(value).unwrap()))
    .collect::<HashMap<String, String>>())
  .unwrap_or(HashMap::<String, String>::new()))
}