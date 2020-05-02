cmake_minimum_required (VERSION 3.6)
project(Rie)

add_executable(Rie Rie.cpp)

include_directories(".")

add_subdirectory(dependencies/sleepy-discord)
add_subdirectory(dependencies/postgres-cxx-client)
add_subdirectory(dependencies/dotenv)


target_link_libraries(Rie sleepy-discord)
target_link_libraries(Rie postgres-cxx-client)
target_link_libraries(Rie dotenv)