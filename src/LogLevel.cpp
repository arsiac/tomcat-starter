//
// Created by arsia on 2021/8/12.
//

#include "LogLevel.h"

/*********************
 * LogLevel > Static *
 *********************/
const tms::LogLevel tms::LogLevel::DEBUG("DEBUG", 10); /* NOLINT */
const tms::LogLevel tms::LogLevel::INFO("INFO", 20); /* NOLINT */
const tms::LogLevel tms::LogLevel::WARN("WARN", 30); /* NOLINT */
const tms::LogLevel tms::LogLevel::ERROR("ERROR", 40); /* NOLINT */

/**************************
 * LogLevel > Constructor *
 **************************/
tms::LogLevel::LogLevel(const char *levelName, int level): name(levelName), code(level) {}

tms::LogLevel::LogLevel(): LogLevel("INVALID", 0) {}

/******************************
 * LogLevel > Getter & Setter *
 ******************************/
const std::string &tms::LogLevel::getName() const {
    return name;
}

void tms::LogLevel::setName(const std::string &_name) {
    LogLevel::name = _name;
}

int tms::LogLevel::getCode() const {
    return code;
}

void tms::LogLevel::setCode(int _code) {
    LogLevel::code = _code;
}

/***********************
 * LogLevel > Operator *
 ***********************/
bool tms::operator>(const tms::LogLevel &l, const tms::LogLevel &r) {
    return l.getCode() > r.getCode();
}

bool tms::operator>=(const tms::LogLevel &l, const tms::LogLevel &r) {
    return l.getCode() >= r.getCode();
}

bool tms::operator<(const tms::LogLevel &l, const tms::LogLevel &r) {
    return l.getCode() < r.getCode();
}

bool tms::operator<=(const tms::LogLevel &l, const tms::LogLevel &r) {
    return l.getCode() <= r.getCode();
}

bool tms::operator==(const tms::LogLevel &l, const tms::LogLevel &r) {
    return l.getCode() == r.getCode();
}
