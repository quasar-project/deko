use std::{
  env,
  fs
};
use std::fs::File;
use std::io::{
  Read,
  Write
};
use std::path::Path;
use anyhow::{
  Context,
  Error
};
use serde_derive::{
  Deserialize,
  Serialize
};
use colored::Colorize;
use crate::log;

pub const CONFIG_DIRECTORY: &str = "config";
pub const CONFIG_FILENAME: &str = "cfg-deko.yml";

pub const DEFAULT_JPEG_MAX_METADATA_LENGTH: usize = 1024;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config
{
  pub jpeg: JpegConfig
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct JpegConfig
{
  pub max_metadata_length: usize
}

impl Default for JpegConfig
{
  fn default() -> Self
  {
    Self
    {
      max_metadata_length: DEFAULT_JPEG_MAX_METADATA_LENGTH
    }
  }
}

impl Default for Config
{
  fn default() -> Self
  {
    Self
    {
      jpeg: JpegConfig::default()
    }
  }
}

impl Config
{
  pub fn from_file() -> Result<Self, Error>
  {
    let path = env::current_dir()?
      .join(CONFIG_DIRECTORY)
      .join(CONFIG_FILENAME)
      .into_os_string()
      .into_string()
      .expect("failed to get config path");
    let exists = Path::new(&path)
      .exists();
    match exists {
      true => {
        log!("found existing config file at {}/{}", CONFIG_DIRECTORY, CONFIG_FILENAME);
        let mut file = File::open(&path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        let config = serde_yaml::from_str(&buf)?;
        log!("loaded config from {}/{}", CONFIG_DIRECTORY, CONFIG_FILENAME);
        Ok(config)
      },
      false => {
        log!("config file at {}/{} not found, creating new config", CONFIG_DIRECTORY, CONFIG_FILENAME);
        let config = Self::default();
        let buf = serde_yaml::to_string(&config)?;
        fs::create_dir_all(
          env::current_dir()?
            .join(CONFIG_DIRECTORY)
        )?;
        let mut file = File::create(&path)?;
        file.write_all(buf.as_bytes())?;
        log!("created new config file at {}/{}", CONFIG_DIRECTORY, CONFIG_FILENAME);
        Ok(config)
      }
    }
  }
}