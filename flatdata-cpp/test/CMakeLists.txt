file(GLOB TEST_FLATDATA_SOURCES "*.cpp" "../vendor/catch_amalgamated.cpp")

flatdata_generate_source(generate_flatdata_test_code
    ${CMAKE_CURRENT_SOURCE_DIR}/test_structures.flatdata
    ${CMAKE_CURRENT_BINARY_DIR}/generated/test_structures.hpp)

flatdata_generate_source(generate_flatdata_test_code2
    ${CMAKE_CURRENT_SOURCE_DIR}/test_structures2.flatdata
    ${CMAKE_CURRENT_BINARY_DIR}/generated/test_structures2.hpp)

flatdata_generate_source(generate_flatdata_test_case_ranges
    ${CMAKE_CURRENT_SOURCE_DIR}/../../test_cases/archives/ranges.flatdata
    ${CMAKE_CURRENT_BINARY_DIR}/generated/ranges.hpp)

# Import feature test cases: generate both imported and root schemas
# The root schema (main) produces #include "types.h", so both files must be
# in the same generated directory.
flatdata_generate_source(generate_import_simple_types
    ${CMAKE_CURRENT_SOURCE_DIR}/../../test_cases/imports/simple/types.flatdata
    ${CMAKE_CURRENT_BINARY_DIR}/generated/imports/simple/types.h)

flatdata_generate_source(generate_import_simple_main
    ${CMAKE_CURRENT_SOURCE_DIR}/../../test_cases/imports/simple/main.flatdata
    ${CMAKE_CURRENT_BINARY_DIR}/generated/imports/simple/main.h)

flatdata_generate_source(generate_import_cross_ns_other
    ${CMAKE_CURRENT_SOURCE_DIR}/../../test_cases/imports/cross_namespace/other.flatdata
    ${CMAKE_CURRENT_BINARY_DIR}/generated/imports/cross_namespace/other.h)

flatdata_generate_source(generate_import_cross_ns_main
    ${CMAKE_CURRENT_SOURCE_DIR}/../../test_cases/imports/cross_namespace/main.flatdata
    ${CMAKE_CURRENT_BINARY_DIR}/generated/imports/cross_namespace/main.h)

add_executable(flatdata_test ${TEST_FLATDATA_SOURCES})
add_dependencies(flatdata_test
    generate_flatdata_test_code
    generate_flatdata_test_code2
    generate_flatdata_test_case_ranges
    generate_import_simple_types
    generate_import_simple_main
    generate_import_cross_ns_other
    generate_import_cross_ns_main
)

target_include_directories(flatdata_test
    PRIVATE ${Boost_INCLUDE_DIRS}
    PRIVATE ${CMAKE_CURRENT_BINARY_DIR}/generated
    PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/../vendor
)

target_compile_definitions(flatdata_test PRIVATE CATCH_CONFIG_CONSOLE_WIDTH=120)

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
