use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
pub enum ImageFormat
{
  Png,
  Jpeg,
  #[default] Unknown
}

impl Display for ImageFormat
{
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "{:?}", self)
  }
}

impl From<&str> for ImageFormat
{
  fn from(value: &str) -> Self
  {
    match value
    {
      "png" => Self::Png,
      "jpg" | "jpeg" => Self::Jpeg,
      _ => Self::Unknown
    }
  }
}