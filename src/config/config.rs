use crate::config::DecoderConfig;
use crate::config::GeneralConfig;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Config
{
  pub general_config: GeneralConfig,
  pub decoder_config: DecoderConfig,
}

impl Default for Config
{
  fn default() -> Self
  {
    Self
    {
      general_config: GeneralConfig::default(),
      decoder_config: DecoderConfig::default()
    }
  }
}