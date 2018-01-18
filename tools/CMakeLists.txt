install(
    DIRECTORY ./
    DESTINATION share/here/routing/flatdata/tools
    COMPONENT test_routing_tools_component
    FILES_MATCHING PATTERN *.py
)

