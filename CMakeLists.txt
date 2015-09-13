cmake_minimum_required(VERSION 3.2)
project(TIG-100)

include_directories(include)

link_libraries(ncursesw)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11")
set(EXECUTABLE_OUTPUT_PATH ${PROJECT_SOURCE_DIR})

set(SOURCE_FILES src/main.cpp src/Cell.cpp include/Cell.hpp src/CodeCell.cpp include/CodeCell.hpp include/termcap.hpp src/Grid.cpp include/Grid.hpp)
add_executable(TIG-100 ${SOURCE_FILES})
