use std::{fs::File, io::Read, path::Path};

use super::Error;

pub enum FileType {
  Config,
  Filter,
  Fallback,
}

impl ToString for FileType {
  fn to_string(&self) -> String {
    String::from(match self {
      Self::Config => "Config",
      Self::Filter => "Filter",
      Self::Fallback => "Fallback",
    })
  }
}

pub struct ConfigFile<'a> {
  path: &'a Path,
  file_type: FileType,
}

impl<'a> ConfigFile<'a> {
  pub fn new(path_string: &'a str, file_type: FileType) -> Result<Self, Error> {
    let path = Path::new(path_string);
    
    if !path.is_file() {
      return Err(Error::Str("Provied path is not a file."));
    }

    Ok(Self {
      path,
      file_type,
    })
  }

  pub fn read(self) -> Result<String, Error> {
    Ok(match (self.file_type, self.path.extension()
    .map(|os_str| os_str.to_str().unwrap_or_default())
    .ok_or(Error::Str("File extension not present."))?) {
      (FileType::Config, "yaml" | "yml")
      | (FileType::Filter, "jq")
      | (FileType::Fallback, "json") => {
        let mut file = File::open(self.path)?;
        let mut file_content = String::new();
        file.read_to_string(&mut file_content)?;
        file_content
      },
      (file_type, _) => Err(Error::String(
        format!("{} file `{}` extension invalid.",
          file_type.to_string(), self.path.to_str().unwrap())))?,
    })
  }
}