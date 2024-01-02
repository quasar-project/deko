use anyhow::Error;

pub trait FromFile
{
  fn from_file(path: &str) -> Result<Self, Error> where Self: Sized;
}

