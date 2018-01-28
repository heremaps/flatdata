find_package(Sphinx REQUIRED)
find_package(Doxygen REQUIRED)

if(NOT DEFINED SPHINX_THEME)
    set(SPHINX_THEME alabaster)
endif()

if(NOT DEFINED SPHINX_THEME_DIR)
    set(SPHINX_THEME_DIR)
endif()

set(BINARY_BUILD_DIR "${CMAKE_CURRENT_BINARY_DIR}/_build")
set(SPHINX_CACHE_DIR "${CMAKE_CURRENT_BINARY_DIR}/_doctrees")
set(SPHINX_HTML_DIR "${CMAKE_CURRENT_BINARY_DIR}/html")

set(FLATDATA_SOURCE_ROOT "${CMAKE_CURRENT_SOURCE_DIR}/..")
set(FLATDATA_CPP_DOXY_ROOT "${CMAKE_CURRENT_BINARY_DIR}/_doxygen/flatdata-cpp/xml")

configure_file(
    "${CMAKE_CURRENT_SOURCE_DIR}/conf.py.in"
    "${CMAKE_CURRENT_BINARY_DIR}/src/conf.py"
@ONLY)

set(DOXYGEN_GENERATE_HTML NO)
set(DOXYGEN_GENERATE_XML YES)
set(DOXYGEN_OUTPUT_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}/_doxygen/flatdata-cpp")
set(DOXYGEN_EXTRACT_PRIVATE NO)
set(DOXYGEN_EXTRACT_ANON_NSPACES NO)
set(DOXYGEN_HIDE_UNDOC_MEMBERS YES)
set(DOXYGEN_HIDE_UNDOC_CLASSES YES)
set(DOXYGEN_HIDE_FRIEND_COMPOUNDS YES)
set(DOXYGEN_WARNINGS NO)
set(DOXYGEN_EXCLUDE_PATTERNS "*/internal/*")

doxygen_add_docs(
    flatdata_cpp_docs
    "${CMAKE_CURRENT_SOURCE_DIR}/../flatdata-cpp/include/flatdata"
    COMMENT "Generate doxygen output for flatdata-cpp"
)

add_custom_target(docs ALL DEPENDS flatdata_cpp_docs)

add_custom_command(TARGET docs
    PRE_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy_directory
    ${CMAKE_CURRENT_SOURCE_DIR}/src ${CMAKE_CURRENT_BINARY_DIR}/src
)

add_custom_command(TARGET docs
    COMMAND sphinx-apidoc
    -o "${CMAKE_CURRENT_BINARY_DIR}/src/flatdata-py"
    --no-headings
    "${CMAKE_CURRENT_SOURCE_DIR}/../flatdata-py"
)

add_custom_command(TARGET docs
    COMMAND ${SPHINX_EXECUTABLE}
    -q -b html
    -E
    -c "${CMAKE_CURRENT_BINARY_DIR}/src"
    -d "${SPHINX_CACHE_DIR}"
    "${CMAKE_CURRENT_BINARY_DIR}/src"
    "${SPHINX_HTML_DIR}"
    COMMENT "Building HTML documentation with Sphinx"
)
