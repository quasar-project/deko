use std::fmt::Display;
use bincode::Decode;
use endian_codec::{
  DecodeBE,
  PackedSize
};
use serde_derive::{
  Deserialize,
  Serialize
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
#[derive(Debug, PartialEq, Serialize, Deserialize, Decode)]
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
  checksum: u16
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
