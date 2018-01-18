set(CHECK_REQUIREMENTS_SCRIPT_PATH ${CMAKE_CURRENT_LIST_DIR})

# Check if all requirements in the given Python requirements file are installed.
#
# @param PATH_TO_REQUIREMENTS_TXT Path to requirements.txt.
# @param RESULT_VAR TRUE if successful, otherwise FALSE
#
function(ensure_python3_requirements PATH_TO_REQUIREMENTS_TXT RESULT_VAR)
    find_program(PYTHON3_EXECUTABLE python3)
    if (NOT PYTHON3_EXECUTABLE)
        message(FATAL_ERROR "Python3 NOT found.")
    endif()
    execute_process(
        COMMAND ${PYTHON3_EXECUTABLE}
        ${CHECK_REQUIREMENTS_SCRIPT_PATH}/check_requirements.py
        ${PATH_TO_REQUIREMENTS_TXT}
        RESULT_VARIABLE CHECK_RESULT
        OUTPUT_QUIET
        ERROR_QUIET
    )
    if (${CHECK_RESULT} STREQUAL 0)
        set(${RESULT_VAR} "TRUE" PARENT_SCOPE)
    else()
        set(${RESULT_VAR} "FALSE" PARENT_SCOPE)
    endif()
endfunction(ensure_python3_requirements)
