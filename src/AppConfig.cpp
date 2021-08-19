//
// Created by arsia on 2021/8/12.
//

#include "AppConfig.h"
#include <cstdlib>
#include <algorithm>
#include "FileUtils.h"

/*
 * tools
 */
bool endWith(const std::string &src, const std::string &end) {
    if (src.empty() || end.empty()) {
        return false;
    }

    return !src.compare(src.length() - end.length(), end.length(), end);
}

/***************************
 * AppConfig > Constructor *
 ***************************/
tms::AppConfig::AppConfig(const boost::filesystem::path &configFile) {
    log = SimpleLogFactory::getLogger("AppConfig");

    configFilePath = configFile;
    commonLogLevel = LogLevel::DEBUG;

    const char *javaHome = getenv(ENV_JAVA_HOME.c_str());
    if (javaHome != nullptr) {
        commonJavaHome = boost::filesystem::path(javaHome);
    }

    const char *jvmOpts = getenv(ENV_JAVA_OPTS.c_str());
    if (jvmOpts != nullptr) {
        commonJvmOptions = std::string(jvmOpts);
    }

    defaultHttpPort = 8080;
    defaultShutPort = 1099;
}

/***********************
 * AppConfig > Private *
 ***********************/

bool tms::AppConfig::initConfigTree() {
    if (!FileUtils::checkFile(configFilePath)) {
        log->error("check configuration file failed.");
        return false;
    }

    try {
        boost::property_tree::ini_parser::read_ini(configFilePath.string(), configTree);
        return true;
    } catch (std::exception &e) {
        log->error() << "read configuration file error: " << e.what() << std::endl;
        return false;
    }
}

bool tms::AppConfig::initLogLevel(const boost::property_tree::ptree &commonTree) {
    if (!commonTree.count(CG_COMMON_LOG_LEVEL)) {
        return true;
    }

    std::string logLevelStr = commonTree.get<std::string>(CG_COMMON_LOG_LEVEL);

    // to upper case
    std::transform(logLevelStr.begin(), logLevelStr.end(), logLevelStr.begin(), ::toupper);

    if (tms::LogLevel::DEBUG.getName() == logLevelStr) {
        commonLogLevel = tms::LogLevel::DEBUG;
    } else if (tms::LogLevel::INFO.getName() == logLevelStr) {
        commonLogLevel = tms::LogLevel::INFO;
    } else if (tms::LogLevel::WARN.getName() == logLevelStr) {
        commonLogLevel = tms::LogLevel::WARN;
    } else if (tms::LogLevel::ERROR.getName() == logLevelStr) {
        commonLogLevel = tms::LogLevel::ERROR;
    } else {
        log->error() << "config option error: " << CG_GROUP_COMMON << " > " << CG_COMMON_LOG_LEVEL
                     << " = " << logLevelStr << std::endl;
        return false;
    }
    return true;
}

bool tms::AppConfig::initJavaHome(const boost::property_tree::ptree &commonTree) {
    if (!commonTree.count(CG_COMMON_JAVA_HOME)) {
        if (commonJavaHome.empty()) {
            log->error() << "Java home cannot be configured, please set environment variable " << ENV_JAVA_HOME
                         << "or set `" << CG_COMMON_JAVA_HOME << "` under `" << CG_GROUP_COMMON << "` group."
                         << std::endl;
            return false;
        }
        log->debug() << "Java home: " << commonJavaHome.string() << std::endl;
        return true;
    }

    boost::filesystem::path javaHomePath = boost::filesystem::path(commonTree.get<std::string>("java_home"));
    if (!FileUtils::checkDirectory(javaHomePath)) {
        log->error() << "config option error: " << CG_GROUP_COMMON << " > " << CG_COMMON_JAVA_HOME
                     << " = " << javaHomePath.string() << std::endl;
        return false;
    } else {
        commonJavaHome = javaHomePath;
    }
    log->info() << "Java home: " << commonJavaHome.string() << std::endl;
    return true;
}

bool tms::AppConfig::initJvmOpts(const boost::property_tree::ptree &commonTree) {
    if (commonTree.count(CG_GLOBAL_JVM_OPTS)) {
        std::string jvmOpts = commonTree.get<std::string>(CG_GLOBAL_JVM_OPTS);
        if (!commonJvmOptions.empty()) {
            commonJvmOptions.append(" ");
        }
        commonJvmOptions.append(jvmOpts);
    }
    log->info() << "Global JVM Options: " << commonJvmOptions << std::endl;
    return true;
}

bool tms::AppConfig::initTomcat(const boost::property_tree::ptree &commonTree) {
    if (commonTree.count(CG_COMMON_TOMCAT)) {
        std::string tomcatStr = commonTree.get<std::string>(CG_COMMON_TOMCAT);
        boost::filesystem::path tomcatPath(tomcatStr);
        if (!FileUtils::checkDirectory(tomcatPath)) {
            log->error() << "config option error: " << CG_GROUP_COMMON << " > " << CG_COMMON_TOMCAT
                         << " = " << tomcatStr << std::endl;
            return false;
        } else {
            commonTomcatLocation = tomcatPath;
        }
    } else {
        log->error() << "config option `" << CG_COMMON_TOMCAT << "` under `" << CG_GROUP_COMMON
                     << "` group is required." << std::endl;
        return false;
    }

    log->info() << "Tomcat: " << commonTomcatLocation.string() << std::endl;

    // default http port
    if (commonTree.count(CG_GLOBAL_HTTP)) {
        int httpPort = commonTree.get<int>(CG_GLOBAL_HTTP);
        if (httpPort <= 0) {
            log->error() << "config option error: " << CG_GROUP_COMMON << " > " << CG_GLOBAL_HTTP
                         << " = " << httpPort << std::endl;
            return false;
        } else {
            defaultHttpPort = httpPort;
        }
    }

    log->info() << "Default Http Port: " << defaultHttpPort << std::endl;

    // default shut port
    if (commonTree.count(CG_GLOBAL_SHUT)) {
        int shutPort = commonTree.get<int>(CG_GLOBAL_SHUT);
        if (shutPort <= 0) {
            log->error() << "config option error: " << CG_GROUP_COMMON << " > " << CG_GLOBAL_SHUT
                         << " = " << shutPort << std::endl;
            return false;
        } else {
            defaultShutPort = shutPort;
        }
    }

    log->info() << "Default Shutdown Port: " << defaultShutPort << std::endl;
    return true;
}

bool tms::AppConfig::initWebAppCommon(const std::string &name, const boost::property_tree::ptree &webTree) {
    webApp.setName(name);
    // http port
    if (webTree.count(CG_GLOBAL_HTTP)) {
        int port = webTree.get<int>(CG_GLOBAL_HTTP);
        if (port <= 0) {
            log->error() << "config option error: " << name << " > " << CG_GLOBAL_HTTP
                         << " = " << port << std::endl;
            return false;
        }
        webApp.setHttpPort(port);
    } else {
        webApp.setHttpPort(defaultHttpPort);
        log->warn() << "Web Application `" << name << "` will use default http port("
                    << defaultHttpPort << ")." << std::endl;
    }

    log->info() << "Web Application `" << name
                << "` Http Port: " << webApp.getHttpPort() << std::endl;

    // shutdown port
    if (webTree.count(CG_GLOBAL_SHUT)) {
        int port = webTree.get<int>(CG_GLOBAL_SHUT);
        if (port <= 0) {
            log->error() << "config option error: " << name << " > " << CG_GLOBAL_SHUT << " = " << port << std::endl;
            return false;
        }
        webApp.setShutPort(port);
    } else {
        webApp.setShutPort(defaultShutPort);
        log->warn() << "Web Application `" << name << "` will use default shutdown port("
                    << defaultShutPort << ")." << std::endl;
    }

    log->info() << "Web Application `" << name
                << "` Shutdown Port: " << webApp.getShutPort() << std::endl;

    // jvm options
    if (webTree.count(CG_GLOBAL_JVM_OPTS)) {
        webApp.setJvmOptions(webTree.get<std::string>(CG_GLOBAL_JVM_OPTS));
    }

    log->info() << "Web Application `" << name
                << "` JVM Options: " << webApp.getJvmOptions() << std::endl;

    return true;
}

bool tms::AppConfig::initSubWebApp(const boost::property_tree::ptree &webTree) {
    for (auto tree = webTree.begin(); tree != webTree.end(); tree++) {
        // context
        if (endWith(tree->first, CG_WEB_CONTEXT_SUFFIX)) {
            std::string appName = tree->first.substr(0, tree->first.length() - CG_WEB_CONTEXT_SUFFIX.length());
            auto subApp = webApp.getOrCreateSubApp(appName);
            subApp->setContext(tree->second.data());
            log->info() << "`" << appName << "` context: " << tree->second.data() << std::endl;
        }
            // war
        else if (endWith(tree->first, CG_WEB_WAR_SUFFIX)) {
            std::string appName = tree->first.substr(0, tree->first.length() - CG_WEB_WAR_SUFFIX.length());
            auto subApp = webApp.getOrCreateSubApp(appName);
            boost::filesystem::path subAppPath(tree->second.data());
            if (!FileUtils::checkFile(subAppPath)) {
                return false;
            }
            subApp->setWar(tree->second.data());
            log->info() << "`" << appName << "` war: " << tree->second.data() << std::endl;
        }
    }

    if (webApp.isEmpty()) {
        log->error("cannot found sub web application.");
        return false;
    }

    return true;
}

bool tms::AppConfig::loadCommon(const boost::property_tree::ptree &commonTree) {

    if (!initLogLevel(commonTree)) {
        return false;
    }

    SimpleLogFactory::setLogLevel(commonLogLevel);
    log->setLogLevel(commonLogLevel);

    if (!initJavaHome(commonTree)) {
        return false;
    }

    initJvmOpts(commonTree);

    if (!initTomcat(commonTree)) {
        return false;
    }

    return true;
}

/**********************
 * AppConfig > Public *
 **********************/

bool tms::AppConfig::load(const std::string &name) {
    if (!initConfigTree()) {
        return false;
    }

    for (auto tree = configTree.begin(); tree != configTree.end(); tree++) {
        log->debug() << "find group: " << tree->first << std::endl;
        // group common
        if (tree->first == "common") {
            if (!loadCommon(tree->second)) {
                return false;
            }
        }
            // group`name`
        else if (tree->first == name) {
            if (!initWebAppCommon(name, tree->second)) {
                return false;
            }

            if (!initSubWebApp(tree->second)) {
                return false;
            }
            return true;
        }
    }

    log->error() << "cannot find this web application: " << name << std::endl;
    return false;
}

/*******************************
 * AppConfig > Getter & Setter *
 *******************************/
const tms::LogLevel &tms::AppConfig::getCommonLogLevel() const {
    return commonLogLevel;
}

void tms::AppConfig::setCommonLogLevel(const tms::LogLevel &logLevel) {
    AppConfig::commonLogLevel = logLevel;
}

const boost::filesystem::path &tms::AppConfig::getCommonJavaHome() const {
    return commonJavaHome;
}

void tms::AppConfig::setCommonJavaHome(const boost::filesystem::path &javaHome) {
    AppConfig::commonJavaHome = javaHome;
}

const std::string &tms::AppConfig::getCommonJvmOptions() const {
    return commonJvmOptions;
}

void tms::AppConfig::setCommonJvmOptions(const std::string &jvmOptions) {
    AppConfig::commonJvmOptions = jvmOptions;
}

const boost::filesystem::path &tms::AppConfig::getCommonTomcatLocation() const {
    return commonTomcatLocation;
}

void tms::AppConfig::setCommonTomcatLocation(const boost::filesystem::path &location) {
    AppConfig::commonTomcatLocation = location;
}

int tms::AppConfig::getDefaultHttpPort() const {
    return defaultHttpPort;
}

void tms::AppConfig::setDefaultHttpPort(int port) {
    AppConfig::defaultHttpPort = port;
}

int tms::AppConfig::getDefaultShutPort() const {
    return defaultShutPort;
}

void tms::AppConfig::setDefaultShutPort(int port) {
    AppConfig::defaultShutPort = port;
}

const tms::WebApplication &tms::AppConfig::getWebApplication() const {
    return webApp;
}

