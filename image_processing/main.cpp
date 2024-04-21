#include <iostream>
#include <opencv2/opencv.hpp>
#include <response.h>
#include <CLI11.hpp>
#include <args.h>

void erode(cv::Mat* frame, int value );
void dilate(cv::Mat* frame, int value );
cv::Mat detect_color(const cv::Mat& frame, const cv::Scalar& color, uint8_t range);
cv::Scalar rgb_to_bgr(const cv::Scalar& color);
cv::Scalar bgr_to_rgb(const cv::Scalar & color);
std::vector<cv::Rect> find_items(cv::Mat* mask);
void process(Response* response, const cv::Mat& frame, const cv::Scalar&  bgr_color, uint8_t range, int dilate_factor, bool debug);
std::vector<cv::Rect> find_items(cv::Mat* mask, bool debug);


//const cv::Scalar NEXT = cv::Scalar(254, 167, 0);
//const cv::Scalar CLEAR = cv::Scalar(226, 78, 27);
//const cv::Scalar TEXT_INPUT = cv::Scalar(0, 255, 0);
//const cv::Scalar ADD_PAYMENT = cv::Scalar(127, 0, 127);
//const cv::Scalar MENU = cv::Scalar(255, 105, 180);
//const cv::Scalar MENU_SELECTED = cv::Scalar(228, 171, 183);



int main(int argc, char* argv[]) {

    // parse args
    auto args = Args();
    if (!args.parse(argc, (argv))) {
        std::cout << "Fail to parse args!" << std::endl;
        return 1;
    }

    // Load the image
    cv::Mat img = cv::imread(args.filename);

    if (img.empty()) {
        std::cout << "Could not read the image" << std::endl;
        return 1;
    }

    Response response(RequestStatus::OK);

    // detect all items in img
    for (const auto& color: args.colors) {
        process(&response, img, color, args.range, args.dilate_factor, args.debug);
    }

    if (args.debug) {
        std::cout << response.items.size() << " items detected!" << std::endl;
    }

    std::cout << response.to_json() << std::endl;

    // Draw bounding rectangles
    for (const auto& item : response.items) {
        cv::rectangle(img, item.bounding_rect, item.color, 2);
    }

    if (args.debug) {
        // Display the result
        cv::imshow("Colored labels detected", img);
        cv::waitKey(0);
    }

    return 0;
}

void process(Response* response, const cv::Mat& frame, const cv::Scalar&  bgr_color, uint8_t range, int dilate_factor, bool debug) {

    cv::Mat mask = detect_color(frame, rgb_to_bgr(bgr_color), range);

    // dilate to merge blobs
    dilate(&mask, dilate_factor);

    // erode to get back to the bounding box
    erode(&mask, dilate_factor);

    // offset 5 pixels
    dilate(&mask, 5);

    if (debug) {
        cv::imshow("Mask", mask);
        cv::waitKey(0);
    }

    std::vector<cv::Rect> items = find_items(&mask, debug);

    for (auto i: items) {
        response->push(Item(bgr_to_rgb(bgr_color), i));
    }
}

void erode(cv::Mat* frame, int value ) {
    value = (2 * value) + 1;
    cv::Size size(value, value);
    cv::Mat tool = cv::getStructuringElement(cv::MORPH_ERODE,size);
    cv::erode(*frame, *frame, tool);
}

void dilate(cv::Mat* frame, int value) {
    value = (2 * value) + 1;
    cv::Size size(value, value);
    cv::Mat tool = cv::getStructuringElement(cv::MORPH_ERODE,size);
    cv::dilate(*frame, *frame, tool);
}

cv::Mat detect_color(const cv::Mat& frame, const cv::Scalar& color, uint8_t range) {
    cv::Scalar upper(color.val[0] + range, color.val[1] + range, color.val[2] + range);
    cv::Scalar lower(color.val[0] - range, color.val[1] - range, color.val[2] - range);

    cv::Mat mask;
    cv::inRange(frame, lower, upper, mask);

    return mask;
}

cv::Scalar rgb_to_bgr(const cv::Scalar& color) {
    return {color.val[2], color.val[1], color.val[0]};
}

cv::Scalar bgr_to_rgb(const cv::Scalar & color) {
    return rgb_to_bgr(color);
}

std::vector<cv::Rect> find_items(cv::Mat* mask) {
    return find_items(mask, false);
}

std::vector<cv::Rect> find_items(cv::Mat* mask, bool debug) {
    // Find contours
    std::vector<std::vector<cv::Point>> contours;
    cv::findContours(*mask, contours, cv::RETR_EXTERNAL, cv::CHAIN_APPROX_SIMPLE);

    // Approximate contours to polygons and get bounding rectangles
    std::vector<cv::Rect> items(contours.size());
    for (size_t i = 0; i < contours.size(); i++) {
        // Approximate the contour to a polygon
        std::vector<cv::Point> contours_poly;
        cv::approxPolyDP(contours[i], contours_poly, 3, true);

        // Get the bounding rectangle
        items[i] = cv::boundingRect(contours_poly);
    }

    if (debug) {
        std::cout << "Found " << items.size() << " items!" << std::endl;
    }

    return  items;
}


