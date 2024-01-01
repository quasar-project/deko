use anyhow::Error;

pub trait Validate
{
  fn validate(&self) -> Result<bool, Error>;
}