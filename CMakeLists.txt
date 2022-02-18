cmake_minimum_required(VERSION 3.8)

project("tomcat-starter")

set(CMAKE_CXX_STANDARD 14)
# set(CMAKE_EXE_LINKER_FLAGS "-static")
set(PROJECT_INCLUDE_DIR "${PROJECT_SOURCE_DIR}/include")
set(EXECUTE_NAME "tms")

file(GLOB_RECURSE SOURCES "src/*.cpp")
message(STATUS "Cpp Sources: ${SOURCES}")
add_executable(${EXECUTE_NAME} ${SOURCES})
include_directories(${PROJECT_INCLUDE_DIR})