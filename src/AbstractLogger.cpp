#include "AbstractLogger.h"

bool AbstractLogger::check(LogLevel l) { return this->level <= l; }

bool AbstractLogger::isDebugEnable() { return check(LogLevel::DEBUG); }

bool AbstractLogger::isInfoEnable() { return check(LogLevel::INFO); }

bool AbstractLogger::isWarnEnable() { return check(LogLevel::WARN); }

bool AbstractLogger::isErrorEnable() { return check(LogLevel::ERROR); }
