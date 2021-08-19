
#ifndef TMS_LOGLEVEL_H
#define TMS_LOGLEVEL_H

#include <string>

namespace tms {
    /**
     * class LogLevel
     *
     * the level of log's output
     */
    class LogLevel {
    private:
        /**
         * level name
         */
        std::string name;

        /**
         * level code
         *
         * LogLevel can be compared by `code`
         */
        int code;

    public:
        /**
         * DEBUG level
         */
        const static LogLevel DEBUG;

        /**
         * INFO level
         */
        const static LogLevel INFO;

        /**
         * WARN level
         */
        const static LogLevel WARN;

        /**
         * ERROR level
         */
        const static LogLevel ERROR;

        /**
         * initialized by `levelName` and `level`
         * @param levelName level name
         * @param level level code
         */
        LogLevel(const char *levelName, int level);

        /**
         * initialized by "INVALID" and 0
         *
         * no-arg constructor outputs any message
         */
        LogLevel();

        const std::string &getName() const;

        void setName(const std::string &_name);

        int getCode() const;

        void setCode(int _code);
    };

    bool operator>(const LogLevel &l, const LogLevel &r);

    bool operator>=(const LogLevel &l, const LogLevel &r);

    bool operator<(const LogLevel &l, const LogLevel &r);

    bool operator<=(const LogLevel &l, const LogLevel &r);

    bool operator==(const LogLevel &l, const LogLevel &r);
}

#endif