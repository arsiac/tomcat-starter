#include "Configuration.h"
#include "os.h"
#include "tmsdef.h"
#include <cstdlib>
#include <cstring>
#include <fstream>
#include <iostream>

const char *Configuration::CG_GRP_GLOBAL = "global";
const char *Configuration::CG_LOG_LEVEL = "log_level";
const char *Configuration::CG_JAVA_HOME = "java_home";
const char *Configuration::CG_JAVA_OPTS = "java_opts";
const char *Configuration::CG_TOMCAT = "tomcat";
const char *Configuration::CG_HTTP_PORT = "http_port";
const char *Configuration::CG_SERVER_PORT = "server_port";
const char *Configuration::CG_JPDA_PORT = "jpda_port";
const char *Configuration::CG_CACHE = "cache_dir";

const char *Configuration::VAL_INVALID_PORT = "0";
const char *Configuration::VAL_ENVIRONMENT = "$env:";

Configuration::Configuration() {
    _logLevel = LogLevel::WARN;
    _javaHome = "";
    _javaOpts = "";
    _tomcat = "";
    _httpPort = "";
    _serverPort = "";
    _jpdaPort = "";
    _cache = "";

    std::string userHome = std::getenv(USER_HOME);
    std::string dir = userHome + FILE_SEPARATOR + CONFIG_DIRECTORY;
    _file = dir + FILE_SEPARATOR + CONFIG_FILE_NAME;

    _valid = true;
    _parser.load(_file);
    if (_parser.isValid()) {
        initialize();
    } else {
        std::cerr << "error: analyze config file failed: " << _file << std::endl;
        _valid = false;
    }
}

void Configuration::initialize() {
    _logLevel = resolveLogLevel(_parser.get(CG_GRP_GLOBAL, CG_LOG_LEVEL, ""));
    _javaHome = resolveValue(_parser.get(CG_GRP_GLOBAL, CG_JAVA_HOME, _javaHome));
    _javaOpts = resolveValue(_parser.get(CG_GRP_GLOBAL, CG_JAVA_OPTS, _javaOpts));
    _tomcat = resolveValue(_parser.get(CG_GRP_GLOBAL, CG_TOMCAT, _tomcat));
    _httpPort = resolveValue(_parser.get(CG_GRP_GLOBAL, CG_HTTP_PORT, _httpPort));
    _serverPort = resolveValue(_parser.get(CG_GRP_GLOBAL, CG_SERVER_PORT, _serverPort));
    _jpdaPort = resolveValue(_parser.get(CG_GRP_GLOBAL, CG_JPDA_PORT, _jpdaPort));
    _cache = resolveValue(_parser.get(CG_GRP_GLOBAL, CG_CACHE, _cache));
}

std::string Configuration::resolveValue(const std::string &_value) {
    if (_value.size() == 0) {
        return _value;
    }
    std::string value = _value;
    size_t envStart = value.find(VAL_ENVIRONMENT);
    if (envStart == std::string::npos) {
        return value;
    }

    // find end of ENV
    size_t envEnd = value.size();
    for (size_t i = envStart + 4; i < value.size(); ++i) {
        char c = value.at(i);
        if (c == ' ' || c == '\t' || c == '\\' || c == '/') {
            envEnd = i;
            break;
        }
    }

    std::string envKey = value.substr(envStart, envEnd - envStart);
    const char *envValue = getenv(envKey.substr(5).c_str());
    return resolveValue(value.replace(envStart, envEnd - envStart, envValue == nullptr ? "" : envValue));
}

void Configuration::resolveWebList(Project &project, const std::string &name) {
    auto node = _parser.getPropertyMap("web \"" + name + '"');

    if (node == nullptr) {
        std::cerr << "error: cannot found war configuration for project \"" << name << "\"." << std::endl;
        project.present(false);
        return;
    }

    std::map<std::string, WebDocument> warMap;
    for (auto &item : *node) {
        size_t splitPos = item.second.find_first_of("|");
        if (splitPos == std::string::npos) {
            project.present(false);
            std::cerr << "error: cannot find '|': " << item.second << std::endl;
            return;
        }
        std::string context = splitPos == 0 ? "/" : item.second.substr(0, splitPos);
        std::string path = item.second.substr(splitPos + 1);
        warMap.insert(std::make_pair(item.first, WebDocument(context, path)));
    }

    project.webMap(warMap);
}

LogLevel Configuration::resolveLogLevel(const std::string &name) {
    if (0 == strcmp(name.c_str(), "debug")) {
        return LogLevel::DEBUG;
    }

    if (0 == strcmp(name.c_str(), "info")) {
        return LogLevel::INFO;
    }

    if (0 == strcmp(name.c_str(), "warn")) {
        return LogLevel::WARN;
    }

    if (0 == strcmp(name.c_str(), "error")) {
        return LogLevel::ERROR;
    }

    std::cerr << "error: log level not support: " << name << std::endl;
    _valid = false;
    return LogLevel::INFO;
}

bool Configuration::isValid() { return _valid; }

LogLevel Configuration::getLogLevel() const { return _logLevel; }

std::string Configuration::getJavaHome() const { return _javaHome; }

std::string Configuration::getJavaOpts() const { return _javaOpts; }

std::string Configuration::getTomcat() const { return _tomcat; }

std::string Configuration::getHttpPort() const { return _httpPort; }

std::string Configuration::getServerPort() const { return _serverPort; }

std::string Configuration::getJpdaPort() const { return _jpdaPort; }

std::string Configuration::getCache() const { return _cache; }

Project Configuration::getProject(const std::string &name) {
    Project project;
    auto node = _parser.getPropertyMap("project \"" + name + '"');

    project.name(name);
    if (node == nullptr) {
        project.present(false);
        return project;
    }

    project.javaHome(resolveValue(getProperty(node, CG_JAVA_HOME, _javaHome)));

    std::string javaOpts = resolveValue(getProperty(node, CG_JAVA_OPTS, "")); 
    if (javaOpts.empty()) {
        javaOpts = _javaOpts;
    } else if (!_javaOpts.empty()) {
        javaOpts = _javaOpts + ' ' + javaOpts;
    }
    project.javaOpts(javaOpts);
    project.tomcat(resolveValue(getProperty(node, CG_TOMCAT, _tomcat)));
    project.httpPort(resolveValue(getProperty(node, CG_HTTP_PORT, _httpPort)));
    project.serverPort(resolveValue(getProperty(node, CG_SERVER_PORT, _serverPort)));
    project.jpdaPort(resolveValue(getProperty(node, CG_JPDA_PORT, _jpdaPort)));

    resolveWebList(project, name);
    return project;
};