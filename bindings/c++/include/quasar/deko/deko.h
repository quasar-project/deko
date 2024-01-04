//
// Created by whs31 on 04.01.2024.
//

#pragma once

#include <string>
#include <qol/expected>

namespace deko
{
  using std::string;
  using std::string_view;

  enum class LogLevel
  {
    Trace,
    Debug,
    Info,
    Warn,
    Error
  };

  auto init_logger(LogLevel level) -> expected<void, string>;
}