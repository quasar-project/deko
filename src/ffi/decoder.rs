#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals, unused)]

use std::ffi::{
  c_char,
  CStr
};

#[no_mangle]
pub extern "C" fn Deko_JpegDecoder_SetTargetDirectory(dir_name: *const c_char)
{
  crate::decoder::jpeg_decoder::JPEG_DECODER
    .lock()
    .unwrap()
    .set_target_directory(
      unsafe { CStr::from_ptr(dir_name).to_str().unwrap() }
    );
}

// todo: set config

#[no_mangle]
pub extern "C" fn Deko_JpegDecoder_DecodeFile(jpeg_path: *const c_char) -> bool
{
  crate::decoder::jpeg_decoder::JPEG_DECODER
    .lock()
    .unwrap()
    .decode_file(unsafe { CStr::from_ptr(jpeg_path).to_str().unwrap() })
    .is_ok()
}

#[no_mangle]
pub extern "C" fn Deko_JpegDecoder_DecodeData(data: *mut u8, size: usize, filename: *const c_char) -> bool
{
  crate::decoder::jpeg_decoder::JPEG_DECODER
    .lock()
    .unwrap()
    .decode_data(
      unsafe { std::slice::from_raw_parts_mut(data, size) },
      unsafe { CStr::from_ptr(filename).to_str().unwrap() }
    ).is_ok()
}