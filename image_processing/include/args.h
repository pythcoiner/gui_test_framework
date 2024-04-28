#pragma once
#include <vector>
#include <opencv2/opencv.hpp>
#include <json.hpp>

struct Args {
    std::string filename;

    uint8_t range{};
    int dilate_factor{};
    std::vector<cv::Scalar> colors;
    bool debug{};

    Args();
    bool parse(int argc, char* argv[]);

    ~Args() = default;
};

