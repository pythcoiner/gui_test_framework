#include <request.h>

std::string status_to_string_msg(RequestStatus status) {
    switch (status) {
        case RequestStatus::OK:
            return "Request => OK";
        case RequestStatus::SIZE_MISSING:
            return "Request::from_json => SIZE missing!";
        case RequestStatus::COLORS_MISSING:
            return "Request::from_json => COLOR missing!";
        case RequestStatus::FRAME_MISSING:
            return "Request::from_json => FRAME missing!";
        case RequestStatus::WRONG_SIZE_FORMAT:
            return "Request::from_json => wrong SIZE format!";
        case RequestStatus::WRONG_COLOR_FORMAT:
            return "Request::from_json => wrong COLOR format!";
        case RequestStatus::WRONG_DATA_SIZE:
            return "Request::from_json => wrong FRAME size!";
        case RequestStatus::WRONG_DATA_FORMAT:
            return "Request::from_json => wrong FRAME format!";
        default:
            return "Unknown status!";
    }
}

std::string status_to_string(RequestStatus* status) {
    switch (*status) {

        case RequestStatus::OK:
            return "OK";
        case RequestStatus::SIZE_MISSING:
            return "SIZE_MISSING";
        case RequestStatus::COLORS_MISSING:
            return "COLOR_MISSING";
        case RequestStatus::FRAME_MISSING:
            return "FRAME_MISSING";
        case RequestStatus::WRONG_SIZE_FORMAT:
            return "WRONG_SIZE_FORMAT";
        case RequestStatus::WRONG_COLOR_FORMAT:
            return "WRONG_COLOR_FORMAT";
        case RequestStatus::WRONG_DATA_SIZE:
            return "WRONG_DATA_SIZE";
        case RequestStatus::WRONG_DATA_FORMAT:
            return "WRONG_DATA_FORMAT";
        default:
            return "Unknown Status";
    }
}

Request::Request(): size(0, 0), status(RequestStatus::OK) {
    this->colors = std::vector<cv::Scalar>();
    this->frame = std::vector<cv::Scalar>();
}

Request Request::from_json(const std::string &json_str) {
    nlohmann::json json = nlohmann::json::parse(json_str);
    Request req;

    // sanity check args
    if (!json.contains(SIZE)) {
        req.status = RequestStatus::SIZE_MISSING;
    } else if (!json.contains(COLORS)) {
        req.status = RequestStatus::COLORS_MISSING;
    } else if (!json.contains(FRAME)) {
        req.status = RequestStatus::FRAME_MISSING;
    } else if (json[SIZE].size() != 2) {
        req.status = RequestStatus::WRONG_SIZE_FORMAT;
    } else if (!json[COLORS].is_array() || json[COLORS].size() %3 != 0)  {
        req.status = RequestStatus::WRONG_COLOR_FORMAT;
    } else if (json[FRAME].size() != ((size_t)(json[SIZE][0]) * (size_t)json[SIZE][0] * 3)) {
        req.status = RequestStatus::WRONG_DATA_SIZE;
    }

    if (req.status != RequestStatus::OK) {
        std::cout << status_to_string_msg(req.status) << std::endl;
        return req;
    }

    // Parse colors
    req.colors.clear();
    for (const auto& color : json[COLORS]) {
        if (color.size() != 3) {
            req.status = RequestStatus::WRONG_COLOR_FORMAT;
            return req;
        }
        cv::Scalar scalarColor(color[0], color[1], color[2]);
        req.colors.push_back(scalarColor);
    }

    // Parse frame, received an already flattened vector
    for (size_t i = 0; i < json[FRAME].size(); i += 3) {

        uint8_t v0, v1, v2;
        // sanity check pixels values
        if (json[FRAME][i] < 255 && json[FRAME][i+1] < 255 && json[FRAME][i+2] < 255) {
            v0 = (uint8_t)(json[FRAME][i]);
            v1 = (uint8_t)(json[FRAME][i+1]);
            v2 = (uint8_t)(json[FRAME][i+2]);
        } else {
            req.status = RequestStatus::WRONG_DATA_FORMAT;
            return req;
        }

        cv::Scalar pixel(v0, v1, v2);
        req.frame.push_back(pixel);
    }

    return req;
}

Request::~Request() = default;

bool Request::is_ok() {
    return this->status == RequestStatus::OK;
}

bool Request::is_err() {
    return !this->is_ok();
}

cv::Mat Request::to_image() {
    // Create an empty cv::Mat of the correct size and type
    cv::Mat image(this->size, CV_8UC3);

    // Get a pointer to the start of the data in the cv::Mat object
    auto* imagePtr = reinterpret_cast<cv::Vec3b*>(image.data);

    // Populate the cv::Mat object using direct memory access
    for (size_t i = 0; i < frame.size(); ++i) {
        // Assign each cv::Scalar to the corresponding location in the image data
        imagePtr[i][0] = static_cast<uint8_t>(frame[i][0]); // Blue
        imagePtr[i][1] = static_cast<uint8_t>(frame[i][1]); // Green
        imagePtr[i][2] = static_cast<uint8_t>(frame[i][2]); // Red
    }

    return image;
}
