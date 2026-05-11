set(FLATDATA_GENERATOR_PATH ${CMAKE_CURRENT_LIST_DIR}/../../../flatdata-generator)

# Generates sources from flatdata schema
#
# @param TARGET_NAME Name of custom target to generate header for schema
# @param SCHEMA_FILENAME path to the flatdata schema
# @param OUTPUT_FILENAME output filename. Generator is forced to output to the given file.
#
function(flatdata_generate_source TARGET_NAME SCHEMA_FILENAME OUTPUT_FILENAME)
    find_program(PYTHON3_EXECUTABLE NAMES python3 python36)
    if (NOT PYTHON3_EXECUTABLE)
        message(FATAL_ERROR "python3 NOT found.")
    endif()

    file(GLOB_RECURSE FLATDATA_GENERATOR_SOURCES ${FLATDATA_GENERATOR_PATH}/**/*.py)
    file(GLOB_RECURSE FLATDATA_GENERATOR_TEMPLATES ${FLATDATA_GENERATOR_PATH}/**/*.jinja2)

    set(DEPFILE ${OUTPUT_FILENAME}.d)
    set(DEPFILE_ARGS)
    if(CMAKE_VERSION VERSION_GREATER_EQUAL "3.20")
        set(DEPFILE_ARGS DEPFILE ${DEPFILE})
    endif()

    add_custom_command(
        OUTPUT ${OUTPUT_FILENAME}
        COMMAND ${PYTHON3_EXECUTABLE} ${FLATDATA_GENERATOR_PATH}/generator.py
        --gen cpp
        --schema ${SCHEMA_FILENAME}
        --output-file ${OUTPUT_FILENAME}
        --depfile ${DEPFILE}
        DEPENDS ${FLATDATA_GENERATOR_SOURCES}
        DEPENDS ${FLATDATA_GENERATOR_TEMPLATES}
        DEPENDS ${SCHEMA_FILENAME}
        ${DEPFILE_ARGS}
        WORKING_DIRECTORY ${GENERATOR_PATH}
        COMMENT "Generating sources from flatdata schema"
    )
    add_custom_target(${TARGET_NAME} DEPENDS ${OUTPUT_FILENAME})
endfunction(flatdata_generate_source)
