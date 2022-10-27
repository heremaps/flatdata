cmake_minimum_required(VERSION 3.2)
project(flatdata-cpp)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}/cmake")
include(flatdata/GenerateSource)

set(CMAKE_CXX_STANDARD 17)

find_package(Boost COMPONENTS system filesystem REQUIRED)
find_package(Threads REQUIRED)

file(GLOB FLATDATA_SOURCE
    "src/*.h"
    "src/*.inl"
    "src/*.cpp")

add_library(flatdata STATIC ${FLATDATA_SOURCE})

set_target_properties(flatdata PROPERTIES POSITION_INDEPENDENT_CODE ON)

target_include_directories(flatdata
    PUBLIC $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
    PUBLIC $<INSTALL_INTERFACE:include>
    PUBLIC ${Boost_INCLUDE_DIRS})

target_link_libraries(flatdata
  ${Boost_LIBRARIES}
  ${CMAKE_THREAD_LIBS_INIT})

enable_testing()
add_subdirectory(test)
add_subdirectory(benchmark)
add_subdirectory(examples)