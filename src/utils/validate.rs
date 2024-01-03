pub trait Validate
{
  fn validate(&self) -> anyhow::Result<bool>;
}