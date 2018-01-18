# Generates sources from flatdata schema
#
# @param TARGET_NAME Name of custom target to generate header for schema
# @param GENERATOR_PATH path to generator code. Has to provide it due to some cmake limitations with functions.
# @param SCHEMA_FILENAME path to the flatdata schema
# @param OUTPUT_FILENAME output filename. Generator is forced to output to the given file.
#
function(flatdata_generate_source TARGET_NAME GENERATOR_PATH SCHEMA_FILENAME OUTPUT_FILENAME)
    find_program(PYTHON3_EXECUTABLE python3)
    if (NOT PYTHON3_EXECUTABLE)
        message(FATAL_ERROR "Python3 NOT found.")
    endif()
    file(GLOB_RECURSE FLATDATA_GENERATOR_SOURCES ${GENERATOR_PATH}/**/*.py)
    file(GLOB_RECURSE FLATDATA_GENERATOR_TEMPLATES ${GENERATOR_PATH}/**/*.jinja2)

    add_custom_command(
        OUTPUT ${OUTPUT_FILENAME}
        COMMAND ${PYTHON_EXECUTABLE} ${GENERATOR_PATH}/app.py
        --gen cpp
        --schema ${SCHEMA_FILENAME}
        --output-file ${OUTPUT_FILENAME}
        DEPENDS ${FLATDATA_GENERATOR_SOURCES}
        DEPENDS ${FLATDATA_GENERATOR_TEMPLATES}
        DEPENDS ${SCHEMA_FILENAME}
        WORKING_DIRECTORY ${GENERATOR_PATH}
        COMMENT "Generating sources from flatdata schema"
    )
    add_custom_target(${TARGET_NAME} DEPENDS ${OUTPUT_FILENAME})
endfunction(flatdata_generate_source)
