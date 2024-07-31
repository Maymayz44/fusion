use sha2::{Digest, Sha256};

pub struct Hasher;

impl Hasher {
  pub fn hash_string(value: String) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(value);
    hasher.finalize().to_vec()
  }
}