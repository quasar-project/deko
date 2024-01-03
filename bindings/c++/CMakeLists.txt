cmake_minimum_required(VERSION 3.25)

project(deko
  VERSION 0.1.0
  DESCRIPTION "quasar images decoding library bindings"
  HOMEPAGE_URL "io.github.quasar-project"
  LANGUAGES C CXX
)

if(NOT CMAKE_CXX_STANDARD)
  set(CMAKE_CXX_STANDARD 23)
  set(CMAKE_CXX_STANDARD_REQUIRED ON)
endif()

set(CMAKE_WINDOWS_EXPORT_ALL_SYMBOLS ON CACHE BOOL "Export all symbols")
set(CMAKE_SHARED_LIBRARY_ENABLE_EXPORTS ON)

set(RLIB_WINDOWS ${PROJECT_SOURCE_DIR}/plugins/Windows/x64/deko.dll)
set(RLIB_LINUX ${PROJECT_SOURCE_DIR}/plugins/Linux/x64/libdeko.so)

if(WIN32)
  if(EXISTS "${RLIB_WINDOWS}")
    message(STATUS "[${PROJECT_NAME}] RLib for Windows found")
  else()
    message(FATAL_ERROR "[${PROJECT_NAME}] RLib for Windows not found!")
  endif()
else()
  if(EXISTS "${RLIB_LINUX}")
    message(STATUS "[${PROJECT_NAME}] RLib for Linux found")
  else()
    message(FATAL_ERROR "[${PROJECT_NAME}] RLib for Linux not found!")
  endif()
endif()

add_library(rust_ffi STATIC IMPORTED GLOBAL)

if(WIN32)
  message(STATUS "[Meridian] Searching for .a in ${RLIB_WINDOWS}")
  set_target_properties(rust_ffi PROPERTIES IMPORTED_LOCATION ${RLIB_WINDOWS})
  set_target_properties(rust_ffi PROPERTIES IMPORTED_IMPLIB ${RLIB_WINDOWS})
else()
  message(STATUS "[Meridian] Searching for .a in ${RLIB_LINUX}")
  set_target_properties(rust_ffi PROPERTIES IMPORTED_LOCATION ${RLIB_LINUX})
endif()

add_library(${PROJECT_NAME} SHARED)

target_sources(${PROJECT_NAME}
#  PUBLIC
#  include/positioning-compact/coordinate
#  include/positioning-compact/coordinate.h
#  include/positioning-compact/global
#  include/positioning-compact/global.h
#  PRIVATE
#  src/c++/abi.h
#  src/c++/coordinate.c++
#  src/c++/global.c++
)

target_link_libraries(${PROJECT_NAME}
#  PUBLIC
#  qol
  PRIVATE
    rust_ffi
)

target_include_directories(${PROJECT_NAME}
  PUBLIC
    include
  PRIVATE
    src/c++
)
