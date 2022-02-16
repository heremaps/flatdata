file(GLOB TEST_FLATDATA_SOURCES "*.cpp")

flatdata_generate_source(generate_flatdata_test_code
    ${CMAKE_CURRENT_SOURCE_DIR}/test_structures.flatdata
    ${CMAKE_CURRENT_BINARY_DIR}/generated/test_structures.hpp)

flatdata_generate_source(generate_flatdata_test_code2
    ${CMAKE_CURRENT_SOURCE_DIR}/test_structures2.flatdata
    ${CMAKE_CURRENT_BINARY_DIR}/generated/test_structures2.hpp)

flatdata_generate_source(generate_flatdata_test_case_ranges
    ${CMAKE_CURRENT_SOURCE_DIR}/../../test_cases/archives/ranges.flatdata
    ${CMAKE_CURRENT_BINARY_DIR}/generated/ranges.hpp)

add_executable(flatdata_test ${TEST_FLATDATA_SOURCES})
add_dependencies(flatdata_test generate_flatdata_test_code generate_flatdata_test_code2 generate_flatdata_test_case_ranges)

target_include_directories(flatdata_test
    PRIVATE ${Boost_INCLUDE_DIRS}
    PRIVATE ${CMAKE_CURRENT_BINARY_DIR}/generated
    PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/../vendor
)

target_link_libraries(flatdata_test
    flatdata
    ${CMAKE_THREAD_LIBS_INIT}
)

add_test(NAME flatdata_test COMMAND flatdata_test)

include(InstallTarget OPTIONAL RESULT_VARIABLE HAS_INSTALL_TARGET)
if(HAS_INSTALL_TARGET)
    install_with_depends(flatdata_test bin lib flatdata_test_component)
endif()

set_property(TARGET flatdata_test PROPERTY FOLDER "flatdata")
