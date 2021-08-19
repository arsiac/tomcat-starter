//
// Created by arsia on 2021/8/12.
//
#include "SimpleLogger.h"
#include <boost/date_time.hpp>

/********************
 * NullStreamBuffer *
 ********************/
std::streamsize tms::NullStreamBuffer::xsputn(const char *s, std::streamsize n) {
    return n;
}

int tms::NullStreamBuffer::overflow(int c) {
    return c;
}

/********************
 * NullOutputStream *
 ********************/
tms::NullOutputStream::NullOutputStream() : std::ostream(new NullStreamBuffer()) {}

/******************************
 * SimpleLogger > Constructor *
 ******************************/
tms::SimpleLogger::SimpleLogger(const tms::LogLevel &_level, const char *_name) {
    level = _level;
    if (_name != nullptr) {
        name = _name;
    } else {
        name = "SimpleLogger";
    }
}

/**************************
 * SimpleLogger > Private *
 **************************/
std::ostream &tms::SimpleLogger::base(const std::string &levelName, const std::string &msg) {
    int max_level_len = 5;
//    time_t timestamp;
//    tm *info;
//    time(&timestamp);
//    info = localtime(&timestamp);
//    char format_time[20];
//
//    strftime(format_time, 20, "%Y-%m-%d %H:%M:%S", info);
    int delta = max_level_len - (int) levelName.length();

    // %Y-%m-%d %H:%M:%S%f
    boost::posix_time::ptime time = boost::posix_time::microsec_clock::local_time();
    std::cout << '[' << time << "]-[";
    for (int i = 0; i < delta; i++) {
        std::cout << " ";
    }

    std::ostream &temp = std::cout << levelName << "] (" << name << ") ";
    return msg.empty() ? temp : temp << msg << std::endl;
}

std::ostream &tms::SimpleLogger::debug(const std::string &msg) {
    return isDebugEnable() ? base(LogLevel::DEBUG.getName(), msg) : nullOut;
}

std::ostream &tms::SimpleLogger::debug() {
    return debug(EMPTY_STRING);
}

std::ostream &tms::SimpleLogger::info(const std::string &msg) {
    return isInfoEnable() ? base(LogLevel::INFO.getName(), msg) : nullOut;
}

std::ostream &tms::SimpleLogger::info() {
    return info(EMPTY_STRING);
}

std::ostream &tms::SimpleLogger::warn(const std::string &msg) {
    return isWarnEnable() ? base(LogLevel::WARN.getName(), msg) : nullOut;
}

std::ostream &tms::SimpleLogger::warn() {
    return warn(EMPTY_STRING);
}

std::ostream &tms::SimpleLogger::error(const std::string &msg) {
    return isErrorEnable() ? base(LogLevel::ERROR.getName(), msg) : nullOut;
}

std::ostream &tms::SimpleLogger::error() {
    return error(EMPTY_STRING);
}

bool tms::SimpleLogger::isDebugEnable() {
    return LogLevel::DEBUG >= level;
}

bool tms::SimpleLogger::isInfoEnable() {
    return LogLevel::INFO >= level;
}

bool tms::SimpleLogger::isWarnEnable() {
    return LogLevel::WARN >= level;
}

bool tms::SimpleLogger::isErrorEnable() {
    return LogLevel::ERROR >= level;
}

void tms::SimpleLogger::setLogLevel(const tms::LogLevel &_level) {
    level = _level;
}

void tms::SimpleLogger::setLogName(const std::string &_name) {
    name = _name;
}

/*****************************
 * SimpleLogFactory > Static *
 *****************************/
std::map<std::string, tms::Logger*> tms::SimpleLogFactory::LOGGER_CACHE = std::map<std::string, tms::Logger*>();

tms::LogLevel tms::SimpleLogFactory::GLOBAL_LOG_LEVEL = LogLevel::DEBUG; /* NOLINT */

tms::Logger *tms::SimpleLogFactory::getLogger(const char *logName) {
    auto itr = LOGGER_CACHE.find(logName);

    if (itr == LOGGER_CACHE.end()) {
        Logger *log = new SimpleLogger(GLOBAL_LOG_LEVEL, logName);
        LOGGER_CACHE.insert(std::make_pair(logName, log));
        return log;
    }

    return itr->second;
}

tms::Logger *tms::SimpleLogFactory::getGlobal() {
    return SimpleLogFactory::getLogger("Global");
}

void tms::SimpleLogFactory::setLogLevel(const LogLevel &level) {
    GLOBAL_LOG_LEVEL = level;
}

void tms::SimpleLogFactory::setTimeFormat(const char *format) {
    auto timeFacet = new boost::posix_time::time_facet(format);
    std::cout.imbue(std::locale(std::cout.getloc(), timeFacet));
}
