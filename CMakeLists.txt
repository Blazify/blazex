cmake_minimum_required(VERSION 3.14)

project(blazex)

include(CheckPIESupported)
check_pie_supported()

add_executable(blazex test.c)
set_target_properties(blazex PROPERTIES POSITION_INDEPENDENT_EXECUTABLE TRUE)
target_link_libraries(blazex ${CMAKE_CURRENT_SOURCE_DIR}/examples/main.o)