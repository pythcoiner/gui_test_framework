#pragma once

#include <vector>
#include <cstdint>
#include <string>
#include <opencv2/opencv.hpp>

struct Item {
    cv::Scalar color;
    uint32_t top{};
    uint32_t bottom{};
    uint32_t left{};
    uint32_t right{};
    cv::Rect bounding_rect;

    Item(cv::Scalar color, cv::Rect bounding_rect);
    std::string to_json();

    ~Item() = default;
};