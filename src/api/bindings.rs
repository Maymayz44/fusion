use std::{env, net::Ipv4Addr, str::FromStr};

use regex::Regex;

pub struct Bindings {
  pub address: Ipv4Addr,
  pub port: u16,
  pub path: String,
}

impl Bindings {
  pub fn env() -> Self {
    let address = env::var("BIND_ADDRESS").unwrap();
    let port = env::var("BIND_PORT").unwrap().parse::<u16>().unwrap();
    let path = env::var("BIND_PATH").unwrap();
    
    assert!(Regex::new(r"^((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)\.?\b){4}$").unwrap().is_match(&address));

    Self {
      address: Ipv4Addr::from_str(&address).unwrap(),
      port: port,
      path: path,
    }
  }

  pub fn full_address(&self) -> String {
    format!("{}:{}", self.address.to_string(), self.port.to_string())
  }
}