//
// Created by arsia on 2021/8/11.
//

#ifndef TMS_LOGGER_H
#define TMS_LOGGER_H

#include <string>
#include <iostream>
#include "LogLevel.h"

namespace tms {
    const std::string EMPTY_STRING;

    class Logger {
    public:
        virtual std::ostream& debug(const std::string &msg) = 0;
        virtual std::ostream& debug() = 0;
        virtual std::ostream& info(const std::string &msg) = 0;
        virtual std::ostream& info() = 0;
        virtual std::ostream& warn(const std::string &msg) = 0;
        virtual std::ostream& warn() = 0;
        virtual std::ostream& error(const std::string &msg) = 0;
        virtual std::ostream& error() = 0;

        virtual bool isDebugEnable() = 0;
        virtual bool isInfoEnable() = 0;
        virtual bool isWarnEnable() = 0;
        virtual bool isErrorEnable() = 0;

        virtual void setLogLevel(const LogLevel &_level) = 0;

        virtual void setLogName(const std::string &_name) = 0;
    };
}

#endif //TMS_LOGGER_H
