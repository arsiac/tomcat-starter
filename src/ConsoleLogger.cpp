#include "ConsoleLogger.h"

ConsoleLogger::ConsoleLogger(const char *name, LogLevel l) {
    this->name = name;
    this->level = l;
}

void ConsoleLogger::debug(const std::string &msg) {
    if (!this->isDebugEnable()) {
        return;
    }
    std::cout << "debug: " << msg << std::endl;
}

void ConsoleLogger::info(const std::string &msg) {
    if (!this->isInfoEnable()) {
        return;
    }
    std::cout << msg << std::endl;
}

void ConsoleLogger::warn(const std::string &msg) {
    if (!this->isWarnEnable()) {
        return;
    }
    std::cout << "warn: " << msg << std::endl;
}

void ConsoleLogger::error(const std::string &msg) {
    if (!this->isErrorEnable()) {
        return;
    }
    std::cerr << "error: " << msg << std::endl;
}