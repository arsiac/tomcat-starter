#pragma once

#include "AbstractLogger.h"
#include <iostream>

class ConsoleLogger : public AbstractLogger {
public:
    ConsoleLogger(const char *name, LogLevel l);

    void debug(const std::string &msg) override;

    void info(const std::string &msg) override;

    void warn(const std::string &msg) override;

    void error(const std::string &msg) override;
};