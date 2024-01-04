//
// Created by whs31 on 04.01.2024.
//

#pragma once

#include <string>
#include <qol/qol>
#include <qol/expected>

namespace deko
{
  using std::string;
  using std::string_view;

  using buffer = std::vector<u8>;

  class JpegDecoder
  {
    public:
      static void set_target_directory(string_view path);
      static auto decode_file(string_view path) -> expected<void, string>;
      static auto decode_raw_data(const buffer& data, string_view filename) -> expected<void, string>;
  };
}