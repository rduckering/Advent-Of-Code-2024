cmake_minimum_required(VERSION 3.24)

set(PROJECT AdventOfCode2024)
project(${PROJECT} VERSION 0.0.1)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Add the executable
add_executable(${PROJECT}
        main.cpp
        Day1.cpp
        Day1.h Day2.cpp Day2.h Utils.cpp Utils.h Day3.cpp Day3.h)
