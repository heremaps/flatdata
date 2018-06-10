cmake_minimum_required(VERSION 3.2)
project(flatdata-cpp)

set(CMAKE_CXX_STANDARD 11)

find_package(Boost COMPONENTS system filesystem REQUIRED)
find_program(FLATDATA_RE2C_EXECUTABLE re2c)

file(GLOB FLATDATA_SOURCE
    "src/*.h"
    "src/*.inl"
    "src/*.cpp")

list(FILTER FLATDATA_SOURCE EXCLUDE REGEX "[.]re2c[.]cpp$")

if (FLATDATA_RE2C_EXECUTABLE)
    list(FILTER FLATDATA_RE2C_EXECUTABLE EXCLUDE REGEX "[.]no-re2c[.]cpp")
    set(re2c_in "${CMAKE_CURRENT_SOURCE_DIR}/src/SchemaUtils.re2c.cpp")
    set(re2c_out "${CMAKE_CURRENT_BINARY_DIR}/SchemaUtils.cpp")
    add_custom_command(
        OUTPUT "${re2c_out}"
        DEPENDS "${FLATDATA_RE2C_EXECUTABLE}"
        DEPENDS "${re2c_in}"
        COMMAND "${FLATDATA_RE2C_EXECUTABLE}"
            # When checking in the file add option
            # --no-debug-info
            # to prevent checking in full paths and line information.
            --no-generation-date
            # When debugging uncomment the following option to generate more
            # readable but slightly slower code.
            --bit-vectors
            --start-conditions
            --input custom
            -o "${re2c_out}"
            "${re2c_in}"
        COMMENT "RE2C ${re2c_in}"
    )

    list(APPEND FLATDATA_SOURCE "${re2c_out}")
endif()

add_library(flatdata STATIC ${FLATDATA_SOURCE})

set_target_properties(flatdata PROPERTIES POSITION_INDEPENDENT_CODE ON)

target_include_directories(flatdata
    PUBLIC $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
    PUBLIC $<INSTALL_INTERFACE:include>
    PUBLIC ${Boost_INCLUDE_DIRS})

target_link_libraries(flatdata
  ${Boost_LIBRARIES})

add_subdirectory(test)
