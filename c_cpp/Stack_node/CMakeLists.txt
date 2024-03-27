cmake_minimum_required(VERSION 3.27)
project(Stack_node C)

set(CMAKE_C_STANDARD 11)

include_directories(.)

add_executable(Stack_node
        main.c
        Stack.c
)
