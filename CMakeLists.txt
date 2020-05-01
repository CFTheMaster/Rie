cmake_minimum_required (VERSION 3.6)
project(Rie)

add_executable(Rie Rie.cpp)

add_subdirectory(dependencies/sleepy-discord)
add_subdirectory(dependencies/postgres-cxx-client)
add_subdirectory(dependencies/dotenv)
add_subdirectory(CommandHandler)
add_subdirectory(Database)


target_link_libraries(Rie sleepy-discord)
target_link_libraries(Rie postgres-cxx-client)
target_link_libraries(Rie dotenv)
target_link_libraries(Rie CommandHandler)
target_link_libraries(Rie Database)
