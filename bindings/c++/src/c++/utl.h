//
// Created by user on 04.01.2024.
//

#pragma once

#include <qol/expected>

// #define ffi_bool_to_expected_void_string(expr, errc) if(const auto result = expr; not result) \
//   return unexpected(errc); return {};

namespace ffi::utils
{
  template<typename TSuccess, typename TFailure>
  [[nodiscard]] auto ok_or(const bool expr, const TFailure& errc) -> expected<TSuccess, TFailure>
  {
    if(const auto result = expr; not result)
      return unexpected(errc);
    return {};
  }
}