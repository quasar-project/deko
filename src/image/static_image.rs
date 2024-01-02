use std::{env, fs};
use std::path::Path;
use anyhow::{Context, Error};
use colored::Colorize;
use crate::{Config, CONFIG, log, utils};
use crate::datagrams::jpeg;
use crate::image::FromFile;

#[derive(Debug, Clone)]
pub struct StaticImage
{
  pub filename: String,
  pub extension: String,
  pub source_path: String,
  pub processed_path: String,
  pub metadata: jpeg::datagram::Metadata,
  pub mercator_zoom_level: f32,
  pub size: utils::Size<u32>
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
    let extension = Path::new(path)
      .extension()
      .context("extension parsing error")?
      .to_os_string()
      .into_string()
      .expect("osstring conversion error");
    log!("processing image {} ({})", &filename.bold().yellow(), &extension.bold().cyan());
    let target_path_folder = env::current_dir()?
      .join(cfg.cache.cache_directory)
      .join(cfg.cache.images_subdirectory)
      .into_os_string()
      .into_string()
      .expect("osstring conversion error");
    fs::create_dir_all(&target_path_folder)?;
    let target_path = Path::new(&target_path_folder)
      .join(&filename)
      .with_extension(&extension)
      .into_os_string()
      .into_string()
      .expect("osstring conversion error");
    fs::copy(path, &target_path)?;
    log!("target path for image: {}", target_path);
    let metadata = jpeg::decode::decode_image(path)?;
    let mercator_zoom_level = utils::mercator::mercator_zoom_level(
      metadata.latitude,
      metadata.dx as f64
    ) as f32;
    let size = imagesize::size(path)?;
    let size = utils::Size::new(size.width as u32, size.height as u32);
    log!("image size: {}", size.to_string().blue().bold());
    Ok(Self {
      filename,
      extension,
      source_path: String::from(path),
      processed_path: target_path,
      metadata,
      mercator_zoom_level,
      size
    })
  }
}

impl StaticImage
{
  fn cut_image(&self) -> Result<&Self, Error>
  {
    Ok(self)
  }
}