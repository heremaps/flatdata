cmake_minimum_required(VERSION 3.2)
set_directory_properties(PROPERTIES CLEAN_NO_CUSTOM 1)
include(ExternalProject)
find_package(Git REQUIRED)

ExternalProject_Add(
    ext_gtest
    PREFIX ${CMAKE_BINARY_DIR}/vendor/gtest
    GIT_REPOSITORY https://github.com/google/googletest.git
    GIT_TAG release-1.8.0
    CMAKE_ARGS
        -DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE}
        -DCMAKE_INSTALL_PREFIX:PATH=<INSTALL_DIR>
        -DCMAKE_C_COMPILER=${CMAKE_C_COMPILER}
        -DCMAKE_CXX_COMPILER=${CMAKE_CXX_COMPILER}
        -DCMAKE_CXX_FLAGS=${CMAKE_CXX_FLAGS}
    LOG_DOWNLOAD ON
    UPDATE_COMMAND ""
)

ExternalProject_Get_Property(ext_gtest install_dir)
set(GTEST_INCLUDE_DIRS ${install_dir}/include
    CACHE INTERNAL "Path to include folder for GTest"
)
set(GTEST_LIBRARIES ${install_dir}/lib/libgtest.a
    CACHE INTERNAL "Path to libraries for GTest"
)
