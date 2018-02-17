add_compile_options(-std=c++11)

function (flatdata_make_example PATH)
    get_filename_component(EXAMPLE_DIR ${PATH} DIRECTORY)
    get_filename_component(EXAMPLE_NAME ${PATH} NAME)

    include(flatdata/GenerateSource)
    flatdata_generate_source(generate_flatdata_${EXAMPLE_NAME}
        ${CMAKE_CURRENT_SOURCE_DIR}/../generator
        ${CMAKE_CURRENT_SOURCE_DIR}/${PATH}.flatdata
        ${CMAKE_CURRENT_BINARY_DIR}/generated/${PATH}.hpp)

    add_executable(${EXAMPLE_NAME} ${PATH}.cpp)
    add_dependencies(${EXAMPLE_NAME} generate_flatdata_${EXAMPLE_NAME})

    target_include_directories(${EXAMPLE_NAME}
        PRIVATE ${Boost_INCLUDE_DIRS}
        PRIVATE ${CMAKE_CURRENT_BINARY_DIR}/generated/${EXAMPLE_DIR})

    target_link_libraries(${EXAMPLE_NAME} flatdata)
endfunction(flatdata_make_example)

flatdata_make_example(binary_layout)
flatdata_make_example(fibonacci)
flatdata_make_example(graph)
flatdata_make_example(coappearances/coappearances)
