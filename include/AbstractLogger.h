#pragma once

#include "Logger.h"

class AbstractLogger : public Logger {
private:
    bool check(LogLevel l);

protected:
    std::string name;
    LogLevel level;

public:
    bool isDebugEnable() override;

    bool isInfoEnable() override;

    bool isWarnEnable() override;

    bool isErrorEnable() override;
};