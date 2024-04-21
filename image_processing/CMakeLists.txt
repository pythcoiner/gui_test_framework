cmake_minimum_required(VERSION 3.5)
project(image_processing)

set(CMAKE_CXX_STANDARD 17)

# Include FetchContent module for downloading dependencies
include(FetchContent)

# Declare the nlohmann_json library from its GitHub repository
FetchContent_Declare(
        json
        GIT_REPOSITORY https://github.com/nlohmann/json.git
        GIT_TAG v3.10.5  # Use a specific release tag
)

# Make the nlohmann_json library available
FetchContent_MakeAvailable(json)

# Find OpenCV package
find_package(OpenCV REQUIRED)

# Add executable
add_executable(image_processing main.cpp
        include/request.h
        include/response.h
        src/request.cpp
        src/response.cpp
        include/item.h
        src/item.cpp
        include/CLI11.hpp
        include/args.h
        src/args.cpp)

# Include directories
target_include_directories(image_processing PRIVATE
        ${OpenCV_INCLUDE_DIRS}
        ${CMAKE_CURRENT_SOURCE_DIR}/include
)

# Link libraries with the executable
target_link_libraries(image_processing
        ${OpenCV_LIBS}
        nlohmann_json::nlohmann_json
)
