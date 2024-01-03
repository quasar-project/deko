use std::fmt::Display;
use anyhow::anyhow;

#[derive(Debug, Default, PartialOrd, PartialEq, Eq, Clone, Copy)]
pub enum ImageType
{
  Jpeg,
  BinaryStream,
  #[default] Invalid
}

impl Display for ImageType
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}", self) }
}

impl ImageType
{
  pub fn from_extension(ext: &str) -> anyhow::Result<Self>
  {
    match ext {
      "jpg" | "jpeg" => Ok(Self::Jpeg),
      "bin" => Ok(Self::BinaryStream),
      _ => Err(anyhow!("invalid image type extension: {}", ext))
    }
  }

  pub fn extension(&self) -> anyhow::Result<&'static str>
  {
    match self {
      Self::Jpeg => Ok("jpg"),
      Self::BinaryStream => Ok("bin"),
      Self::Invalid => Err(anyhow!("invalid image type"))
    }
  }
}