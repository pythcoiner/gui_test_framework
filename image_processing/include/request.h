#pragma once

#include <opencv2/core.hpp>
#include <json.hpp>
#include <vector>
#include <string>
#include <array>
#include <iostream>
#include <cstdint>

// Json field names
const std::string SIZE = "size";
const std::string COLORS = "colors";
const std::string FRAME = "frame";

// Status / Parsing errors
enum class RequestStatus : uint8_t {
    OK,
    SIZE_MISSING,
    COLORS_MISSING,
    FRAME_MISSING,
    WRONG_SIZE_FORMAT,
    WRONG_COLOR_FORMAT,
    WRONG_DATA_SIZE,
    WRONG_DATA_FORMAT,
};

std::string status_to_string_msg(RequestStatus status);
std::string status_to_string(RequestStatus* status);

class Request {
private:
    cv::Size size;
    std::vector<cv::Scalar> colors;
    std::vector<cv::Scalar> frame;
    RequestStatus status;
public:

    Request();

    bool is_ok();
    bool is_err();
    cv::Mat to_image();

    static Request from_json(const std::string& json_str );

    ~Request();
};
