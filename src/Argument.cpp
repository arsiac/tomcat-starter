#include "Argument.h"
#include <cstring>
#include <iostream>

const char *Argument::CLI_PROJECT_LONG = "--project";
const char *Argument::CLI_PROJECT = "-p";
const char *Argument::CLI_WAR_LONG = "--web-document";
const char *Argument::CLI_WAR = "-w";
const char *Argument::CLI_HTTP_PORT = "--http-port";
const char *Argument::CLI_SERVER_PORT = "--server-port";
const char *Argument::CLI_JPDA_PORT = "--jpda-port";
const char *Argument::CLI_DEBUG_MODE_LONG = "--debug-mode";
const char *Argument::CLI_DEBUG_MODE = "-d";
const char *Argument::CLI_NEW_WINDOW_LONG = "--new-window";
const char *Argument::CLI_NEW_WINDOW = "-n";
const char *Argument::CLI_VERSION = "--version";
const char *Argument::CLI_CONFIG_TEMPLATE = "--config-template";
const char *Argument::CLI_CLEAN_LONG = "--clean";

const char *Argument::VAL_INVALID_PORT = "0";

const char *Argument::VERSION = "tms version 2.0.0-beta";

const char *Argument::CONFIG_TEMPLATE =
    "; ${HOME}/.tms/config.ini\n"
    "[global]\n"
    "; debug, info, warn, error and close\n"
    "log_level = warn\n"
    "java_home = $env:JAVA_HOME\n"
    "java_opts = $env:JAVA_OPTS -XX:+HeapDumpOnOutOfMemoryError -XX:-OmitStackTraceInFastThrow\n"
    "tomcat = $env:CATALINA_HOME\n"
    "http_port = 8080\n"
    "server_port = 8005\n"
    "jpda_port = 5005\n"

#if defined(WIN32) || defined(WIN64)
    "cache_dir = $env:USERPROFILE\\.tms\n\n"
#else
    "cache_dir = $env:HONE/.tms\n\n"
#endif

    "[project \"web\"]\n"
    "java_home = xxx\n"
    "java_opts = xxx\n"
    "tomcat = xxx\n"
    "http_port = xxx\n"
    "server_port = xxx\n"
    "jpda_port = xxx\n\n"
    "[web \"web\"]\n"
    "; name = contextPath|webDocumentPath\n"
    "war1 = war1|/path/to/xxx.war\n"
    "xxx = xxx_context|/path/to/xxx.war\n";

const char *Argument::USAGE = "Usage: tms [OPTION]...\n"
                              "Run or debug web project by tomcat.\n\n"
                              "Mandatory arguments to long options are mandatory for short options too.\n"
                              "-p, --project         project name.\n"
                              "-w, --web-document    web doc that need to be run.\n"
                              "-d, --debug-mode      open tomcat jpda mode.\n"
                              "-n, --new-window      run web project in new window.\n"
                              "--http-port           Tomcat HTTP port.\n"
                              "--server-port         Tomcat Server port.\n"
                              "--jpda-port           Tomcat JPDA port.\n"
                              "--config-template     show configuration file template.\n"
                              "--clean               clean cache directory.\n"
                              "--version             Tomcat Starter version.\n";

Argument::Argument(int argc, const char *argv[]) {
    _debugMode = false;
    _newWindow = false;
    _valid = true;
    _httpPort = VAL_INVALID_PORT;
    _serverPort = VAL_INVALID_PORT;
    _jpdaPort = VAL_INVALID_PORT;
    _project = "";
    analyze(argc, argv);
}

void Argument::validate() {
    if (_project.length() == 0) {
        std::cout << "error: project name is required(--project, -p)." << std::endl;
        _valid = false;
    }

    if (_docList.empty()) {
        std::cout << "error: web document is required(--web-document, -w)." << std::endl;
        _valid = false;
    }
}

bool Argument::checkOption(const char *o1, const char *o2, const char *op) {
    bool matchOptionOne = o1 != nullptr && strcmp(o1, op) == 0;
    bool matchOptionTwo = o2 != nullptr && strcmp(o2, op) == 0;
    if (matchOptionOne || matchOptionTwo) {
        return true;
    }
    return false;
}

void Argument::analyze(int argc, const char *argv[]) {
    // no options
    if (argc < 2) {
        _valid = false;
        std::cout << USAGE << std::endl;
        return;
    }

    // show version
    if (0 == strcmp(CLI_VERSION, argv[1])) {
        _valid = false;
        std::cout << VERSION << std::endl;
        return;
    }

    // show configuration template
    if (0 == strcmp(CLI_CONFIG_TEMPLATE, argv[1])) {
        _valid = false;
        std::cout << CONFIG_TEMPLATE << std::endl;
        return;
    }

    for (int i = 1; i < argc; i++) {
        int res = resolve(argv[i], i == argc - 1 ? nullptr : argv[i + 1]);
        if (res == -1) {
            _valid = false;
            return;
        }

        if (res == -2) {
            return;
        }

        i += res;
    }

    validate();
}

int Argument::resolve(const char *current, const char *next) {
    if (checkOption(CLI_CLEAN_LONG, nullptr, current)) {
        this->_clean = true;
        return -2;
    }
    if (checkOption(CLI_PROJECT_LONG, CLI_PROJECT, current)) {
        this->_project = next;
        return 1;
    }

    if (checkOption(CLI_WAR_LONG, CLI_WAR, current)) {
        if (next == nullptr) {
            std::cerr << "error: war package name cannot be null." << std::endl;
            return -1;
        }
        this->_docList.push_back(next);
        return 1;
    }

    if (checkOption(CLI_HTTP_PORT, nullptr, current)) {
        if (next == nullptr) {
            std::cerr << "error: http port cannot be null." << std::endl;
            return -1;
        }
        this->_httpPort = next;
        return 1;
    }

    if (checkOption(CLI_SERVER_PORT, nullptr, current)) {
        if (next == nullptr) {
            std::cerr << "error: server port cannot be null." << std::endl;
            return -1;
        }
        this->_serverPort = next;
        return 1;
    }

    if (checkOption(CLI_JPDA_PORT, nullptr, current)) {
        if (next == nullptr) {
            std::cout << "error: jpda port cannot be null." << std::endl;
            return -1;
        }
        this->_jpdaPort = next;
        return 1;
    }

    if (checkOption(CLI_DEBUG_MODE, CLI_DEBUG_MODE_LONG, current)) {
        this->_debugMode = true;
        return 0;
    }

    if (checkOption(CLI_NEW_WINDOW, CLI_NEW_WINDOW_LONG, current)) {
        this->_newWindow = true;
        return 0;
    }

    std::cerr << "error: option [" << current << "] not support." << std::endl;
    return -1;
}

std::string Argument::projectName() const { return this->_project; }

std::list<std::string> Argument::docList() const { return this->_docList; }

std::string Argument::httpPort() const { return this->_httpPort; }

std::string Argument::serverPort() const { return this->_serverPort; }

std::string Argument::jpdaPort() const { return this->_jpdaPort; }

bool Argument::debugMode() const { return this->_debugMode; }

bool Argument::otherWindow() const { return this->_newWindow; }

bool Argument::isValid() const { return this->_valid; }

bool Argument::isClean() const { return this->_clean; }
