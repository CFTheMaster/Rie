cmake_minimum_required (VERSION 3.6)
project(Rie)

add_executable(Rie Rie.cpp)

add_subdirectory(dependencies/sleepy-discord)
add_subdirectory(dependencies/soci)

target_link_libraries(Rie sleepy-discord)
target_link_libraries(Rie soci)