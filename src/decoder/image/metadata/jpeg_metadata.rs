use std::fmt::Display;
use std::mem::size_of;
use bincode::{
  Decode,
  Encode
};
use endian_codec::{
  DecodeBE,
  PackedSize
};
use log::debug;
use serde_derive::{
  Deserialize,
  Serialize
};
use crate::utils::{
  Checksum,
  Validate
};

/// JPEG metadata header. Serialized in big endian
#[derive(Debug, PartialEq, Serialize, Deserialize, DecodeBE, PackedSize)]
pub struct MetadataHeader
{
  /// Metadata marker (JPEG_METADATA_MARKER)
  pub marker: u16,

  /// Metadata length in bytes
  pub length: u16
}

/// JPEG metadata
/// Serialized in little endian, except for the header.
#[derive(Debug, PartialEq, Serialize, Deserialize, Decode, Encode, Clone)]
pub struct Metadata
{
  /// Latitude in WGS84 datum of image anchor point (°)
  pub latitude: f64,

  /// Longitude in WGS84 datum of image anchor point (°)
  pub longitude: f64,

  /// Horizontal resolution (m/pixel)
  pub dx: f32,

  /// Vertical resolution (m/pixel)
  pub dy: f32,

  /// Near edge of image offset (m)
  pub x0: f32,

  /// Frame offset (m)
  pub y0: f32,

  /// Azimuth of image (°)
  pub azimuth: f32,

  /// Drift angle relative to azimuth (°)
  pub drift_angle: f32,

  /// Image width (m)
  pub lx: f32,

  /// Image height (m)
  pub ly: f32,

  /// Arc divergence (°)
  pub div: f32,

  /// Velocity in the moment of capture (m/s)
  pub velocity: f32,

  /// Altitude in the moment of capture rel. to sea level (m)
  pub altitude: f32,

  /// Frequency Interpolation Coefficient
  pub fic: f32,

  /// Image offset from time of capture (s)
  pub time_offset: f32,

  /// Total capture duration (s)
  pub time_duration: f32,

  /// Reserved
  reserved_1: f32,

  /// Reserved
  reserved_2: f32,

  /// Image mode
  pub mode: u8,

  /// Image type (0 means telescopic)
  pub image_type: u8,

  /// Reserved
  reserved_3: u32,

  /// Reserved
  reserved_4: u32,

  /// Checksum (CRC16)
  pub(crate) checksum: u16
}

impl Metadata
{
  pub(crate) fn with_fixed_nans(&self) -> Self
  {
    // iterate over numeric fields and check for nans
    let mut new = self.clone();
    for field in [
      &mut new.dx,
      &mut new.dy,
      &mut new.x0,
      &mut new.y0,
      &mut new.azimuth,
      &mut new.drift_angle,
      &mut new.lx,
      &mut new.ly,
      &mut new.div,
      &mut new.velocity,
      &mut new.altitude,
      &mut new.fic,
      &mut new.time_offset,
      &mut new.time_duration,
    ] {
      if field.is_nan() {
        *field = 0.0;
        debug!("fixed nan in {}", field);
      }
    }
    new
  }
}

impl Display for MetadataHeader
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "marker: 0x{:x}, length: {}", self.marker, self.length)
  }
}

impl Display for Metadata
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "[{}° {}°; {}/{} m/px {}m {}m, {}°({}°), {}x{} m, {}°, {} m/s {} m alt, {} kr, \
               {} offset, {} dur, {} mode, {} type], checksum: 0x{:x}",
           self.latitude,
           self.longitude,
           self.dx,
           self.dy,
           self.x0,
           self.y0,
           self.azimuth,
           self.drift_angle,
           self.lx,
           self.ly,
           self.div,
           self.velocity,
           self.altitude,
           self.fic,
           self.time_offset,
           self.time_duration,
           self.mode,
           self.image_type,
           self.checksum
    )
  }
}

impl Checksum<u16> for Metadata
{
  fn checksum(&self) -> anyhow::Result<u16>
  {
    let buf_len = size_of::<Metadata>() - size_of::<u16>();
    let mut buf = vec![0u8; buf_len];
    bincode::encode_into_std_write(&self, &mut buf, bincode::config::standard()).unwrap();
    buf = buf[0..buf_len].to_vec();
    let crc = crate::utils::crc16(buf.as_slice());
    Ok(crc)
  }
}

impl Validate for Metadata
{
  fn validate(&self) -> anyhow::Result<bool>
  {
    Ok(self.checksum()? == self.checksum)
  }
}