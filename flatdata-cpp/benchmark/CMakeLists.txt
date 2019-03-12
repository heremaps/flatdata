if (NOT ${UNIX})
    return()
endif()

file(GLOB BENCHMARK_FLATDATA_SOURCES "*.cpp")

flatdata_generate_source(generate_flatdata_benchmark_code
    ${CMAKE_CURRENT_SOURCE_DIR}/graph.flatdata
    ${CMAKE_CURRENT_BINARY_DIR}/generated/graph.hpp)

add_executable(flatdata_benchmark ${BENCHMARK_FLATDATA_SOURCES})
add_dependencies(flatdata_benchmark generate_flatdata_benchmark_code)

target_include_directories(flatdata_benchmark
    PRIVATE ${GTEST_INCLUDE_DIRS}
    PRIVATE ${Boost_INCLUDE_DIRS}
    PRIVATE ${CMAKE_CURRENT_BINARY_DIR}/generated
    PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/../vendor)

target_link_libraries(flatdata_benchmark
    flatdata)

include(InstallTarget OPTIONAL RESULT_VARIABLE HAS_INSTALL_TARGET)
if(HAS_INSTALL_TARGET)
    install_with_depends(flatdata_benchmark bin lib flatdata_benchmark_component)
    install_with_depends(run.py bin lib flatdata_benchmark_component)
endif()

set_property(TARGET flatdata_benchmark PROPERTY FOLDER "flatdata")
