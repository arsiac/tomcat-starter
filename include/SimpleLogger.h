
#ifndef TMS_SIMPLELOG_H
#define TMS_SIMPLELOG_H

#include <string>
#include <iostream>
#include <map>
#include <ctime>
#include "LogLevel.h"
#include "Logger.h"

namespace tms {
    /**
     * class NullStreamBuffer
     *
     * accept any message and store nothing.
     */
    class NullStreamBuffer : public std::streambuf {
        std::streamsize xsputn(const char *s, std::streamsize n) override;

        int overflow(int c) override;
    };

    /**
     * class NullOutputStream
     *
     * accept any message and output nothing.
     */
    class NullOutputStream : public std::ostream {
    public:
        NullOutputStream();
    };

    /**
     * class SimpleLogger
     *
     * output message to console.
     */
    class SimpleLogger : public Logger {
    private:
        /**
         * log level
         */
        LogLevel level;

        /**
         * logger name
         */
        std::string name;

        /**
         * null output stream
         */
        NullOutputStream nullOut;

        /**
         * assemble and output the message.
         *
         * @param levelName log level name
         * @param msg message
         * @return output stream
         */
        std::ostream &base(const std::string &levelName, const std::string &msg);

    public:
        /**
         * initialized by log level and log name
         *
         * @param _level log level
         * @param _name log name
         */
        SimpleLogger(const tms::LogLevel &_level, const char *_name);

        std::ostream &debug(const std::string &msg) override;

        std::ostream &debug() override;

        std::ostream &info(const std::string &msg) override;

        std::ostream &info() override;

        std::ostream &warn(const std::string &msg) override;

        std::ostream &warn() override;

        std::ostream &error(const std::string &msg) override;

        std::ostream &error() override;

        bool isDebugEnable() override;

        bool isInfoEnable() override;

        bool isWarnEnable() override;

        bool isErrorEnable() override;

        void setLogLevel(const LogLevel &_level) override;

        void setLogName(const std::string &_name) override;
    };

    /**
     * class SimpleLogFactory
     */
    class SimpleLogFactory {
    private:
        /**
         * Logger cache
         */
        static std::map<std::string, Logger *> LOGGER_CACHE;

        /**
         * global log level
         */
        static LogLevel GLOBAL_LOG_LEVEL;

    public:
        /**
         * get or create a logger.
         *
         * @param logName log name
         * @return logger
         */
        Logger static *getLogger(const char *logName);

        /**
         * get or create a global logger.
         *
         * @return global logger
         */
        Logger static *getGlobal();

        /**
         * set global log level.
         *
         * @param level level
         */
        void static setLogLevel(const LogLevel &level);

        /**
         * set time format
         * @param format format string
         */
        void static setTimeFormat(const char *format);
    };
}

#endif