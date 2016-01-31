cmake_minimum_required(VERSION 3.2)
project(TIG-100)

include_directories(include)

link_libraries(ncurses)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11")
set(EXECUTABLE_OUTPUT_PATH ${PROJECT_SOURCE_DIR})

file(STRINGS sources.mk SOURCE_FILES)
add_executable(TIG-100 ${SOURCE_FILES})
