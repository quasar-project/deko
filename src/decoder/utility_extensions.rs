use std::fmt::Display;
use anyhow::anyhow;

#[derive(Debug, Default, PartialOrd, PartialEq, Eq, Clone, Copy)]
pub enum Extension
{
  Json,
  #[default] Invalid
}

impl Display for Extension
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{:?}", self) }
}

impl Extension
{
  pub fn from_extension(ext: &str) -> anyhow::Result<Self>
  {
    match ext {
      "json" => Ok(Self::Json),
      _ => Err(anyhow!("invalid extension: {}", ext))
    }
  }

  pub fn extension(&self) -> anyhow::Result<&'static str>
  {
    match self {
      Self::Json => Ok("json"),
      Self::Invalid => Err(anyhow!("invalid image type"))
    }
  }
}