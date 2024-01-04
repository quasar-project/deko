//
// Created by whs31 on 04.01.2024.
//

#pragma once

#include "../../include/quasar/deko/decoder.h"
#include "abi.h"
#include "utl.h"

void deko::JpegDecoder::set_target_directory(const string_view path) { ffi::Deko_JpegDecoder_SetTargetDirectory(path.data()); }

auto deko::JpegDecoder::decode_file(const string_view path) -> expected<void, string>
{
  return ffi::utils::ok_or<void, string>(
    ffi::Deko_JpegDecoder_DecodeFile(path.data()),
    "failed to decode file"
  );
}

auto deko::JpegDecoder::decode_raw_data(const buffer& data, const string_view filename) -> expected<void, string>
{
  return ffi::utils::ok_or<void, string>(
    ffi::Deko_JpegDecoder_DecodeData(const_cast<uint8_t*>(data.data()), data.size(), filename.data()),
    "failed to decode raw data"
  );
}


