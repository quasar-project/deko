pub const JPEG_HEADER_OFFSET: usize = 20;         /// JPEG Header offset from start to metadata in bytes
pub const JPEG_METADATA_MARKER: u16 = 0xFFE1;     /// JPEG metadata start marker (big endian)

/// JPEG metadata header. Serialized in big endian
pub struct MetadataHeader
{
  pub marker: u16,                      /// Metadata marker (JPEG_METADATA_MARKER)
  pub length: u16                       /// Metadata length in bytes
}

/// JPEG metadata
/// Serialized in little endian, except for the header.
pub struct Metadata
{
  pub header: MetadataHeader,           /// Metadata header: marker and meta length
  pub latitude: f64,                    /// Latitude in WGS84 datum of image anchor point (°)
  pub longitude: f64,                   /// Longitude in WGS84 datum of image anchor point (°)
  pub dx: f32,                          /// Horizontal resolution (m/pixel)
  pub dy: f32,                          /// Vertical resolution (m/pixel)
  pub x0: f32,                          /// Near edge of image offset (m)
  pub y0: f32,                          /// Frame offset (m)
  pub azimuth: f32,                     /// Azimuth of image (°)
  pub drift_angle: f32,                 /// Drift angle relative to azimuth (°)
  pub lx: f32,                          /// Image width (m)
  pub ly: f32,                          /// Image height (m)
  pub div: f32,                         /// Arc divergence (°)
  pub velocity: f32,                    /// Velocity in the moment of capture (m/s)
  pub altitude: f32,                    /// Altitude in the moment of capture rel. to sea level (m)
  pub fic: f32,                         /// Frequency Interpolation Coefficient
  pub time_offset: f32,                 /// Image offset from time of capture (s)
  pub time_duration: f32,               /// Total capture duration (s)
  reserved_1: f32,                      /// Reserved
  reserved_2: f32,                      /// Reserved
  pub mode: u8,                         /// Image mode
  pub image_type: u8,                   /// Image type (0 means telescopic)
  reserved_3: u32,                      /// Reserved
  reserved_4: u32,                      /// Reserved
  checksum: u16                         /// Checksum (CRC16)
}


