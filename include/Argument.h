#pragma once

#include <list>
#include <string>

class Argument {
public:
    static const char *CLI_PROJECT_LONG;
    static const char *CLI_PROJECT;
    static const char *CLI_WAR_LONG;
    static const char *CLI_WAR;
    static const char *CLI_HTTP_PORT;
    static const char *CLI_SERVER_PORT;
    static const char *CLI_JPDA_PORT;
    static const char *CLI_DEBUG_MODE_LONG;
    static const char *CLI_DEBUG_MODE;
    static const char *CLI_NEW_WINDOW_LONG;
    static const char *CLI_NEW_WINDOW;
    static const char *CLI_CONFIG_TEMPLATE;
    static const char *CLI_VERSION;
    static const char *CLI_CLEAN_LONG;

    static const char *VAL_INVALID_PORT;

private:
    static const char *VERSION;
    static const char *CONFIG_TEMPLATE;
    static const char *USAGE;

    std::string _project;

    std::list<std::string> _docList;

    std::string _httpPort;

    std::string _serverPort;

    std::string _jpdaPort;

    bool _valid;

    bool _debugMode;

    bool _newWindow;

    bool _clean;

    void validate();

    bool checkOption(const char *longOption, const char *shortOption, const char *op);

    void analyze(int argc, const char *argv[]);

    int resolve(const char *current, const char *next);

public:
    Argument(int argc, const char *argv[]);

    std::string projectName() const;

    std::list<std::string> docList() const;

    std::string httpPort() const;

    std::string serverPort() const;

    std::string jpdaPort() const;

    bool debugMode() const;

    bool otherWindow() const;

    bool isValid() const;

    bool isClean() const;
};