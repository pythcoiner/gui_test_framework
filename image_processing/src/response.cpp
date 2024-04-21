#include <response.h>

Response::Response(RequestStatus status): status(status) {}

void Response::push(const Item& item) {
    this->items.push_back(item);
}

std::string Response::to_json() {
    std::ostringstream out;
    out << R"({ "status": ")" << status_to_string(&(this->status)) <<
        R"(", "items": [)";

    for (auto item = this->items.begin(); item != this->items.end(); ++item) {
        out << item->to_json();
        if (std::next(item) != items.end()) {
            out << ", ";
        }
    }

    out << " ] }" << std::endl;
    return out.str();
}
