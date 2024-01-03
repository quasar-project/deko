#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageShape
{
  Telescopic,
  Strip,
  Stream,
  Unknown
}

impl From<u8> for ImageShape
{
  fn from(shape: u8) -> Self
  {
    match shape {
      0 => Self::Telescopic,
      1 => Self::Strip,
      _ => Self::Unknown
    }
  }
}