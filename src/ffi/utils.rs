#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals, unused)]

use std::ffi::{
  c_char,
  CStr
};

#[no_mangle]
pub extern "C" fn init_logger(log_level: *const c_char) -> bool
{
  let log_level = unsafe { CStr::from_ptr(log_level) };
  crate::init_logger(log_level.to_str().unwrap_or("debug")).is_ok()
}
