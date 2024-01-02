use std::env;
use std::path::Path;
use anyhow::{Context, Error};
use crate::{Config, CONFIG};
use crate::datagrams::jpeg;
use crate::image::FromFile;

#[derive(Debug, Clone)]
pub struct StaticImage
{
  pub filename: String,
  pub source_path: String,
  pub processed_path: String,
  pub metadata: jpeg::datagram::Metadata
}

impl FromFile for StaticImage
{
  fn from_file(path: &str) -> Result<Self, Error>
  {
    let mut cfg: Config;
    {
      let cfg_lock = CONFIG
        .lock()
        .unwrap();
      cfg = cfg_lock.clone();
    }
    let cfg = cfg;
    let filename = Path::new(path)
      .file_stem()
      .context("filename parsing error")?
      .to_os_string()
      .into_string()
      .expect("osstring conversion error");
    let target_path = env::current_dir()?
      .join(cfg.cache.cache_directory)
      .join(cfg.cache.images_subdirectory)
      .into_os_string()
      .into_string()
      .expect("osstring conversion error");
    let metadata = jpeg::decode::decode_image(path)?;
    Ok(Self {
      filename,
      source_path: String::from(path),
      processed_path: target_path,
      metadata
    })
  }
}

