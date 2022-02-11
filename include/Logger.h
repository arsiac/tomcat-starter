#pragma once

#include <string>

enum class LogLevel {
    DEBUG = 100,
    INFO = 200,
    WARN = 300,
    ERROR = 400,
    CLOSE = 9999
};

class Logger {
public:
    virtual void debug(const std::string &msg) = 0;

    virtual void info(const std::string &msg) = 0;

    virtual void warn(const std::string &msg) = 0;

    virtual void error(const std::string &msg) = 0;

    virtual bool isDebugEnable() = 0;

    virtual bool isInfoEnable() = 0;

    virtual bool isWarnEnable() = 0;

    virtual bool isErrorEnable() = 0;
};