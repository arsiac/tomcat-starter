//
// Created by arsia on 2021/8/17.
//

#include "StringUtils.h"

std::string&
tms::StringUtils::
replaceAll(std::string &source, const std::string &target, const std::string &replacement) {
    if (!source.empty() && !target.empty()) {
        size_t length = target.length(), position;

        while ((position = source.find(target)) != std::string::npos) {
            source.replace(position, length, replacement);
        }
    }
    return source;
}


bool
tms::StringUtils::
endWith(const std::string &source, const std::string &end) {
    return false;
}
