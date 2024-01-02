use std::{
  env,
  fs
};
use std::fmt::{Display, Formatter};
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

/// If set to true, invalid checksum will fail function, otherwise it will only warn
pub const DEFAULT_ALLOW_CHECKSUM_MISMATCH: bool = true;

/// If set to true, radians will be expected in angle fields
pub const DEFAULT_RADIANS: bool = true;

/// If set to true, NaNs will be converted to zeros, otherwise NaNs will remain
pub const DEFAULT_ALLOW_NANS: bool = true;

/// Default directory for cache
pub const DEFAULT_CACHE_DIRECTORY: &str = "cache";

/// Default subdirectory for images in cache
pub const DEFAULT_IMAGES_SUBDIRECTORY: &str = "images";

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Config
{
  pub jpeg: JpegConfig,
  pub cache: CacheConfig,
  pub allow_checksum_mismatch: bool,
  pub radians: bool,
  pub allow_nans: bool
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct JpegConfig
{
  pub max_metadata_length: usize,
  pub header_offset: usize,
  pub metadata_marker: u16
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CacheConfig
{
  pub cache_directory: String,
  pub images_subdirectory: String
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

impl Default for CacheConfig
{
  fn default() -> Self
  {
    Self
    {
      cache_directory: String::from(DEFAULT_CACHE_DIRECTORY),
      images_subdirectory: String::from(DEFAULT_IMAGES_SUBDIRECTORY)
    }
  }
}

impl Default for Config
{
  fn default() -> Self
  {
    Self
    {
      jpeg: JpegConfig::default(),
      cache: CacheConfig::default(),
      allow_checksum_mismatch: DEFAULT_ALLOW_CHECKSUM_MISMATCH,
      radians: DEFAULT_RADIANS,
      allow_nans: DEFAULT_ALLOW_NANS
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

impl Display for CacheConfig
{
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "\tcache folder: \t\t{}\n", self.cache_directory)?;
    write!(f, "\timages subdirectory: \t{}\n", self.images_subdirectory)?;
    Ok(())
  }
}

impl Display for Config
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "  jpeg configuration:\n")?;
    write!(f, "{}", self.jpeg)?;
    write!(f, "{}", self.cache)?;
    write!(f, "  allow checksum mismatch: \t{}\n", self.allow_checksum_mismatch)?;
    write!(f, "  use radians: \t\t\t{}\n", self.radians)?;
    write!(f, "  allow nans: \t\t\t{}\n", self.allow_nans)?;
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
        log!("created new config file at {}/{}", CONFIG_DIRECTORY, CONFIG_FILENAME);
        config.save()?;
        Ok(config)
      }
    }
  }

  pub fn save(&self) -> Result<(), Error>
  {
    let path = env::current_dir()?
      .join(CONFIG_DIRECTORY)
      .join(CONFIG_FILENAME)
      .into_os_string()
      .into_string()
      .expect("failed to get config path");
    let buf = serde_yaml::to_string(&self)?;
    fs::create_dir_all(
      env::current_dir()?
        .join(CONFIG_DIRECTORY)
    )?;
    let mut file = File::create(&path)?;
    file.write_all(buf.as_bytes())?;
    log!("saved config to {}/{}", CONFIG_DIRECTORY, CONFIG_FILENAME);
    Ok(())
  }

  pub fn verbose(&self) -> &Self
  {
    log!("{}", "-- configuration --".to_string().bold().magenta());
    println!("{}", self.to_string().italic());
    self
  }
}