#include "args.h"
#include <CLI11.hpp>

Args::Args() = default;

bool Args::parse(int argc, char *argv[]) {
    CLI::App app{"Colored labels detector"};

    app.add_option("-f,--file", this->filename, "The image to retrieve the colored labels on.")
            ->required()
            ->check(CLI::ExistingFile);  // Ensure the file exists

    app.add_option("-r,--range", this->range, "Color range.")
            ->default_val(10);

    app.add_option("-d,--dilate", this->dilate_factor, "Dilate factor.")
            ->default_val(15);

    std::string json_colors;
    app.add_option("-c,--colors", json_colors, "A JSON list of RGB colors")->required();

    CLI11_PARSE(app, argc, argv);

    nlohmann::json json = nlohmann::json::parse(json_colors);

    if (!json.contains("colors")) {
        std::cout << "Colors key missing in json data!" << std::endl;
        return false;
    } else if (json["colors"].empty()) {
        std::cout << "Colors list is empty!" << std::endl;
        return false;
    }

    for (const auto& color_array : json["colors"]) {
        if (color_array.size() != 3) {
            std::cout << "Each color entry must have exactly three integers." << std::endl;
            return false;
        }

        if (color_array[0] > 255 || color_array[1] > 255 || color_array[2] > 255 ||
        color_array[0] < 0 || color_array[1] < 0 || color_array[2] < 0) {
            std::cout << "Colors value out of range!" << std::endl;
            return false;
        }

        cv::Scalar color(
                color_array[0].get<int>(),
                color_array[1].get<int>(),
                color_array[2].get<int>()
        );
        this->colors.push_back(std::move(color));
    }

    return true;
}
