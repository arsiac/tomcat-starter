//
// Created by arsia on 2021/8/12.
//

#ifndef TMS_APPCONFIG_H
#define TMS_APPCONFIG_H

#include "tmsdef.h"
#include "LogLevel.h"
#include "SimpleLogger.h"
#include "WebApplication.h"
#include <boost/filesystem.hpp>
#include <boost/property_tree/ptree.hpp>
#include <boost/property_tree/ini_parser.hpp>

namespace tms {
    /**
     * class AppConfig
     */
    class AppConfig {
    private:
        boost::filesystem::path configFilePath;
        Logger *log;
        boost::property_tree::ptree configTree;

        LogLevel commonLogLevel;
        boost::filesystem::path commonJavaHome;
        std::string commonJvmOptions;
        boost::filesystem::path commonTomcatLocation;
        int defaultHttpPort;
        int defaultShutPort;
        WebApplication webApp;

        bool initConfigTree();

        bool initLogLevel(const boost::property_tree::ptree &commonTree);

        bool initJavaHome(const boost::property_tree::ptree &commonTree);

        bool initJvmOpts(const boost::property_tree::ptree &commonTree);

        bool initTomcat(const boost::property_tree::ptree &commonTree);
        
        bool initWebAppCommon(const std::string &name, const boost::property_tree::ptree &webTree);

        bool initSubWebApp(const boost::property_tree::ptree &webTree);

        bool loadCommon(const boost::property_tree::ptree &commonTree);

    public:
        explicit AppConfig(const boost::filesystem::path &configFile);

        bool load(const std::string &name);

        const WebApplication &getWebApplication() const;

        const LogLevel &getCommonLogLevel() const;

        void setCommonLogLevel(const LogLevel &logLevel);

        const boost::filesystem::path &getCommonJavaHome() const;

        void setCommonJavaHome(const boost::filesystem::path &javaHome);

        const std::string &getCommonJvmOptions() const;

        void setCommonJvmOptions(const std::string &jvmOptions);

        const boost::filesystem::path &getCommonTomcatLocation() const;

        void setCommonTomcatLocation(const boost::filesystem::path &location);

        int getDefaultHttpPort() const;

        void setDefaultHttpPort(int port);

        int getDefaultShutPort() const;

        void setDefaultShutPort(int port);
    };
}

#endif //TMS_APPCONFIG_H
