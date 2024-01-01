use lazy_static::lazy_static;
use crate::config::Config;

lazy_static!
{
  pub static ref CONFIG: Config = Config::from_file()
    .expect("failed to load config")
    .verbose()
    .clone();
}