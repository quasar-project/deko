#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GeneralConfig
{
  pub angle_unit: AngleUnit,
  pub allow_checksum_mismatch: bool,
  pub fix_nans: bool
}

#[derive(Debug, Clone, Copy)]
pub enum AngleUnit
{
  Radians,
  Degrees
}

impl Default for GeneralConfig
{
  fn default() -> Self
  {
    Self
    {
      angle_unit: AngleUnit::Radians,
      allow_checksum_mismatch: true,
      fix_nans: true
    }
  }
}