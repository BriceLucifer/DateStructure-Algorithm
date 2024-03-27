cmake_minimum_required(VERSION 3.27)
project(linkedlist)

set(CMAKE_CXX_STANDARD 14)

include_directories(.)

add_executable(linkedlist
        main.cpp
        stack.h)
