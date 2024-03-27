cmake_minimum_required(VERSION 3.27)
project(stack_vec)

set(CMAKE_CXX_STANDARD 14)

include_directories(.)

add_executable(stack_vec
        main.cpp
        Stack.h)
