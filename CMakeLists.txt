cmake_minimum_required(VERSION 3.9)
project(flatdata)

include(${CMAKE_BINARY_DIR}/conanbuildinfo.cmake OPTIONAL RESULT_VARIABLE USE_CONAN)
if(USE_CONAN)
    conan_basic_setup()
endif()

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}/cmake")
include(flatdata/PythonRequirements)
ensure_python3_requirements(${CMAKE_CURRENT_LIST_DIR}/generator/requirements.txt HAS_GENERATOR_PKGS)
if(NOT HAS_GENERATOR_PKGS)
    file(READ generator/requirements.txt REQUIREMENTS_CONTENT)
    message(FATAL_ERROR
        "Missing Python3 packages for the Flatdata Generator. "
        "The requirements as specified in `generator/requirements.txt` are:\n"
        "${REQUIREMENTS_CONTENT}")
endif()

enable_testing()

add_subdirectory(flatdata-cpp)
add_subdirectory(flatdata-py)
add_subdirectory(docs)