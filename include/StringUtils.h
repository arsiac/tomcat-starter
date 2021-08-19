//
// Created by arsia on 2021/8/17.
//

#ifndef TMS_STRINGUTILS_H
#define TMS_STRINGUTILS_H

#include <string>

namespace tms {
    class StringUtils {
    public:
        std::string static &replaceAll(std::string &source, const std::string &target, const std::string &replacement);

        bool static endWith(const std::string&source, const std::string &end);

    };
}

#endif //TMS_STRINGUTILS_H
