use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::config::Config;

lazy_static!
{
  pub static ref CONFIG: Mutex<Config> = Mutex::new(Config::from_file()
    .expect("failed to load config")
    .verbose()
    .clone()
  );
}