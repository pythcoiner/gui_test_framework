#include <item.h>
#include <sstream>
#include <utility>

Item::Item(cv::Scalar color, cv::Rect bounding_rect):
           color(std::move(color)), bounding_rect(bounding_rect)  {
    this->top = this->bounding_rect.y;
    this->bottom = this->bounding_rect.y + this->bounding_rect.height;
    this->left = this->bounding_rect.x;
    this->right = this->bounding_rect.x + this->bounding_rect.width;
}

std::string Item::to_json() {
    std::ostringstream out;

    out << "{ \"color\": {"
            "\"r\": " << this->color.val[2] <<
            ", \"g\": " << this->color.val[1] <<
            ", \"b\": " << this->color.val[0] <<
            "}, \"top\": " << this->top <<
            ", \"bottom\": " << this->bottom <<
            ", \"left\": " << this->left <<
            ", \"right\": " << this->right << "}";

    return out.str();
}
