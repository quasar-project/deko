//
// Created by whs31 on 04.01.2024.
//

#include "../../include/quasar/deko/deko.h"
#include "abi.h"

auto deko::init_logger(LogLevel level) -> expected<void, string>
{
  auto level_to_string = [](const LogLevel lvl) -> string_view
  {
    switch(lvl)
    {
      case LogLevel::Trace: return "trace";
      case LogLevel::Debug: return "debug";
      case LogLevel::Warn: return "warn";
      case LogLevel::Error: return "error";
      case LogLevel::Info: [[fallthrough]];
      default: return "info";
    }
  };
  const auto level_string = level_to_string(level);
  if(const auto result = ffi::Deko_InitLogger(level_string.data()); not result)
    return unexpected("failed to initialize logger");
  return {};
}
