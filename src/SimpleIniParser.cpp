#include "SimpleIniParser.h"
#include <cstring>
#include <fstream>
#include <iostream>
#include <string>

SimpleIniParser::SimpleIniParser(const std::string &path) {
    _path = path;
    _valid = parse();
}

SimpleIniParser::SimpleIniParser() { _valid = false; }

SimpleIniParser::~SimpleIniParser() {
    for (auto &node : _resource) {
        delete node.second;
    }
}

std::string getProperty(const std::map<std::string, std::string> *node, const std::string &key,
                        const std::string &defaultValue) {
    if (node == nullptr) {
        return defaultValue;
    }

    auto result = node->find(key);
    if (result == node->end()) {
        return defaultValue;
    }

    return result->second;
}

bool SimpleIniParser::isValid() const { return _valid; }

void SimpleIniParser::load(const std::string &path) {
    _resource.clear();
    _path = path;
    _valid = parse();
}

SimpleIniParser::PropertyMap *SimpleIniParser::getPropertyMap(const std::string &key) const {
    auto result = _resource.find(key);
    if (result == _resource.end()) {
        std::cerr << "error: property map not found: " << key << std::endl;
        return nullptr;
    }

    return result->second;
}

std::string SimpleIniParser::get(const std::string &node, const std::string &key) const {
    PropertyMap *map = getPropertyMap(node);
    auto result = map->find(key);
    if (result == map->end()) {
        std::cerr << "error: key not found: " << key << std::endl;
        return "";
    }

    return result->second;
}

std::string SimpleIniParser::get(const std::string &node, const std::string &key,
                                 const std::string &defaultValue) const {
    std::string value = get(node, key);
    // std::cout << "get(" << node << "," << key << "): " << value << std::endl;
    return value.size() == 0 ? defaultValue : value;
}

bool SimpleIniParser::parse() {
    std::ifstream file(_path);

    if (!file.is_open()) {
        std::cerr << "error: file not found: " << _path << std::endl;
        return false;
    }

    std::string line;
    PropertyMap *currentMap = nullptr;
    while (std::getline(file, line)) {
        // remove blank space
        if (line.size() > 0 && line.at(0) == ' ') {
            line.erase(0, line.find_first_not_of(" "));
        }

        // skip empty lines, comments
        if (line.size() == 0 || line.at(0) == '#' || line.at(0) == ';') {
            continue;
        }

        // group
        if (line.at(0) == '[') {
            std::string node = line.substr(1, line.find_first_of("]") - 1);
            auto res = _resource.find(node);
            if (res == _resource.end()) {
                currentMap = new PropertyMap();
                _resource.insert(ResourceMap::value_type(node, currentMap));
            } else {
                currentMap = res->second;
            }
            continue;
        }

        if (currentMap == nullptr) {
            std::cerr << "error: syntax error > group not found." << std::endl;
            return false;
        }

        size_t equalPos = line.find_first_of("=");
        if (equalPos == std::string::npos) {
            std::cerr << "error: ini syntax error: " << line << std::endl;
            continue;
        }
        std::string key = line.substr(0, equalPos);
        size_t keySpacePos = key.find_first_of(" ");
        if (keySpacePos != std::string::npos) {
            key.erase(key.find_first_of(" "), key.size());
        }

        // remove blank space
        std::string value;
        if (equalPos == line.size() - 1) {
            value = "";
        } else {
            value = line.substr(line.find_first_of("=") + 1, line.size());
        }

        if (!value.empty() && value.at(0) == ' ') {
            size_t valueSpacePos = value.find_first_not_of(" ");
            if (valueSpacePos != std::string::npos) {
                value.erase(0, valueSpacePos);
            } else {
                value = "";
            }
        }

        if (!value.empty() && value.at(value.size() - 1) == ' ') {
            size_t valueNotSpacePos = value.find_last_not_of(" ") + 1;
            if (valueNotSpacePos != std::string::npos) {
                value.erase(valueNotSpacePos, value.size());
            } else {
                value = "";
            }
        }

        // std::cout << "parse(" << key << ", " << value << ")" << std::endl;
        currentMap->insert(PropertyMap::value_type(key, value));
    }

    file.close();
    return true;
}
