#pragma once
#include <vector>
#include <opencv2/opencv.hpp>
#include <nlohmann/json.hpp>

struct Args {
    std::string filename;

    uint8_t range{};
    int dilate_factor{};
    std::vector<cv::Scalar> colors;

    Args();
    bool parse(int argc, char* argv[]);

    ~Args() = default;
};

