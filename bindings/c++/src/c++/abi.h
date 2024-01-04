//
// Created by whs31 on 04.01.2024.
//

#pragma once

#include <qol/platform>

#if defined(LIBRA_OS_WINDOWS)
  #define DEKO_ABI_IMPORT __declspec(dllimport)
#else
  #define DEKO_ABI_IMPORT
#endif

namespace ffi
{
  extern "C"
  {
    DEKO_ABI_IMPORT bool Deko_InitLogger(const char* log_level);

    DEKO_ABI_IMPORT void Deko_JpegDecoder_SetTargetDirectory(const char*);
    DEKO_ABI_IMPORT bool Deko_JpegDecoder_DecodeFile(const char* path);
    DEKO_ABI_IMPORT bool Deko_JpegDecoder_DecodeData(uint8_t* data, size_t size, const char* filename);
  }
}
