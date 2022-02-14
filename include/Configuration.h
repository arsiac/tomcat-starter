#pragma once

#include "Logger.h"
#include "SimpleIniParser.h"
#include "Project.h"
#include <string>
#include <list>

class Configuration {
public:
    static const char *CG_GRP_GLOBAL;
    static const char *CG_LOG_LEVEL;
    static const char *CG_JAVA_HOME;
    static const char *CG_JAVA_OPTS;
    static const char *CG_TOMCAT;
    static const char *CG_HTTP_PORT;
    static const char *CG_SERVER_PORT;
    static const char *CG_JPDA_PORT;
    static const char *CG_CACHE;

    static const char *VAL_INVALID_PORT;
    static const char *VAL_ENVIRONMENT;

private:
    SimpleIniParser _parser;

    bool _valid;

    std::string _file;

    LogLevel _logLevel;

    std::string _javaHome;

    std::string _javaOpts;

    std::string _tomcat;

    std::string _httpPort;

    std::string _serverPort;

    std::string _jpdaPort;

    std::string _cache;

    void initialize();

    LogLevel resolveLogLevel(const std::string &name);

    std::string resolveValue(const std::string &value);

    void resolveWebList(Project &project, const std::string &name);

public:
    Configuration();

    bool isValid();

    LogLevel logLevel() const;

    std::string javaHome() const;

    std::string javaOpts() const;

    std::string tomcat() const;

    std::string httpPort() const;

    std::string serverPort() const;

    std::string jpdaPort() const;

    std::string baseCache() const;

    Project getProject(const std::string &name);
};