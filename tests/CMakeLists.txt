include(flatdata/PythonRequirements)
ensure_python3_requirements(${CMAKE_CURRENT_LIST_DIR}/../flatdata-py/requirements.txt HAS_FLATDATA_PY_PKGS)
if(NOT HAS_FLATDATA_PY_PKGS)
    file(STRINGS ${CMAKE_CURRENT_LIST_DIR}/../flatdata-py/requirements.txt REQUIREMENTS_CONTENT)
    message(WARNING
        "flatdata-py tests are disabled due to the missing Python3 packages. "
        "The required packages are as specified in `flatdata-py/requirements.txt`:\n"
        "${REQUIREMENTS_CONTENT}")
    return()
endif()

find_program(NOSETESTS nosetests)
find_program(PYTHON3_EXECUTABLE python3)

if(NOSETESTS)
    add_test(NAME flatdata_py_test COMMAND ${PYTHON3_EXECUTABLE} -m nose ${CMAKE_CURRENT_LIST_DIR})
    set_property(TEST flatdata_py_test PROPERTY
        ENVIRONMENT "PYTHONPATH=${CMAKE_CURRENT_LIST_DIR}/../flatdata-py")
else()
    message(WARNING "\"nosetests\" not found. Python tests disabled.")
endif()
