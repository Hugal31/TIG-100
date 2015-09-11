cmake_minimum_required(VERSION 3.2)
project(TIG_100)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11")

set(SOURCE_FILES src/main.cpp)
add_executable(TIG_100 ${SOURCE_FILES})
