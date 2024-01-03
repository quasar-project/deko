#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DecoderConfig
{
  pub max_metadata_length: usize,
  pub header_offset: usize,
  pub metadata_marker: u16
}

impl Default for DecoderConfig
{
  fn default() -> Self
  {
    Self
    {
      max_metadata_length: 1024,
      header_offset: 20,
      metadata_marker: 0xFFE1
    }
  }
}