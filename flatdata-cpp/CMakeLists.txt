cmake_minimum_required(VERSION 3.2)
project(flatdata-cpp)

set(CMAKE_CXX_STANDARD 11)

find_package(Boost COMPONENTS system filesystem REQUIRED)

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
  ${Boost_LIBRARIES})

add_subdirectory(test)
add_subdirectory(benchmark)
