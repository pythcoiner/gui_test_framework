cmake_minimum_required(VERSION 3.5)
project(image_processing)

set(CMAKE_CXX_STANDARD 17)

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
        include/json.hpp
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
)
