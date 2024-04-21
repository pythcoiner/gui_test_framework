#pragma once

#include <request.h>
#include <item.h>
#include <sstream>

struct Response {

    RequestStatus status;
    std::vector<Item> items;

    explicit Response(RequestStatus status);
    void push(const Item& item);
    std::string to_json();

    ~Response() = default;
};