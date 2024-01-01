use std::{
  env,
  fs
};
use std::fmt::Display;
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

/// Maximum JPEG metadata length in bytes
pub const DEFAULT_JPEG_MAX_METADATA_LENGTH: usize = 1024;

/// JPEG Header offset from start to metadata in bytes
pub const DEFAULT_JPEG_HEADER_OFFSET: usize = 20;

/// JPEG metadata start marker (big endian)
pub const DEFAULT_JPEG_METADATA_MARKER: u16 = 0xFFE1;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Config
{
  pub jpeg: JpegConfig
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct JpegConfig
{
  pub max_metadata_length: usize,
  pub header_offset: usize,
  pub metadata_marker: u16
}

impl Default for JpegConfig
{
  fn default() -> Self
  {
    Self
    {
      max_metadata_length: DEFAULT_JPEG_MAX_METADATA_LENGTH,
      header_offset: DEFAULT_JPEG_HEADER_OFFSET,
      metadata_marker: DEFAULT_JPEG_METADATA_MARKER
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

impl Display for JpegConfig
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "\tmax metadata length: \t{}\n", self.max_metadata_length)?;
    write!(f, "\theader offset: \t\t{}\n", self.header_offset)?;
    write!(f, "\tmetadata marker: \t0x{:x}\n", self.metadata_marker)?;
    Ok(())
  }
}

impl Display for Config
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "  jpeg configuration:\n")?;
    write!(f, "{}", self.jpeg)?;
    Ok(())
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

  pub fn verbose(&self) -> &Self
  {
    log!("{}", "-- configuration --".to_string().bold().magenta());
    println!("{}", self.to_string().italic());
    self
  }
}