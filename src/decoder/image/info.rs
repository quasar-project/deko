use anyhow::Context;
use log::trace;
use serde_derive::{
  Deserialize,
  Serialize
};
use crate::decoder::image::metadata::jpeg_metadata::Metadata;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageInfo
{
  pub timestamp: u64,
  pub mode: u8,
  pub mercator_zoom_level: f32
}

const MAP_SCALE_RATIO: f64 = 156543.03392;

impl ImageInfo
{
  pub fn new(filename: &str, metadata: &Metadata) -> anyhow::Result<Self>
  {
    let re = regex::Regex::new(r"m([0-9]+)-")?;
    let mode = re
      .captures(filename)
      .and_then(|c| c.get(1))
      .map(|m| m.as_str())
      .and_then(|m| m.parse::<u8>().ok())
      .unwrap_or(0);
    trace!("gathering info from {}", filename);
    let timestamp = chrono::NaiveDateTime::parse_from_str(
      &filename[(filename.find("-").context("failed to find '-' in filename")? + 1)..],
      "%d-%m-%Y_%H-%M-%S"
    )?.timestamp() as u64;
    Ok(Self {
      timestamp,
      mode,
      mercator_zoom_level: Self::mercator_zoom_level(metadata.latitude, metadata.dx as f64)
    })
  }

  fn mercator_zoom_level(lat: f64, m_per_px: f64) -> f32
  {
    (MAP_SCALE_RATIO * lat
      .to_radians()
      .cos() / if m_per_px <= 0.0 { 1.0 } else { m_per_px })
      .log(2.0) as f32
  }
}