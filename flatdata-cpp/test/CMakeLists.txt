find_package(GTest QUIET)
find_package(Threads)
if(NOT GTest_FOUND)
    set(USE_VENDOR_GTEST ON)
    add_subdirectory(${PROJECT_SOURCE_DIR}/vendor/gtest ${CMAKE_CURRENT_BINARY_DIR}/gtest)
endif()

file(GLOB TEST_FLATDATA_SOURCES "*.cpp")

include(flatdata/GenerateSource)
flatdata_generate_source(generate_flatdata_test_code
    ${CMAKE_CURRENT_SOURCE_DIR}/../../generator
    ${CMAKE_CURRENT_SOURCE_DIR}/test_structures.flatdata
    ${CMAKE_CURRENT_BINARY_DIR}/generated/test_structures.hpp)

add_executable(flatdata_test ${TEST_FLATDATA_SOURCES})
add_dependencies(flatdata_test generate_flatdata_test_code)
if(USE_VENDOR_GTEST)
    add_dependencies(flatdata_test ext_gtest)
endif()

target_include_directories(flatdata_test
    PRIVATE ${GTEST_INCLUDE_DIRS}
    PRIVATE ${Boost_INCLUDE_DIRS}
    PRIVATE ${CMAKE_CURRENT_BINARY_DIR}/generated
)

target_link_libraries(flatdata_test
    flatdata
    ${GTEST_LIBRARIES}
    ${CMAKE_THREAD_LIBS_INIT}
)

add_test(NAME flatdata_test COMMAND flatdata_test)

include(InstallTarget OPTIONAL RESULT_VARIABLE HAS_INSTALL_TARGET)
if(HAS_INSTALL_TARGET)
    install_with_depends(flatdata_test bin lib flatdata_test_component)
endif()

set_property(TARGET flatdata_test PROPERTY FOLDER "flatdata")
