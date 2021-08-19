//
// Created by arsia on 2021/8/11.
//

#ifndef TMS_APPLICATION_H
#define TMS_APPLICATION_H
#include <string>
#include <vector>
#include <boost/filesystem.hpp>
#include "Logger.h"
#include "SimpleLogger.h"
#include "AppConfig.h"
#include "TomcatWebApplication.h"

namespace tms {
    class Application {
        typedef boost::filesystem::path path;
    private:
        Logger *log;
        AppConfig *config;

        int httpPort;
        int shutPort;
        bool newWindow;
        std::string webAppName;
        std::vector<std::string> subApps;

    public:
        explicit Application(AppConfig *config);

        ~Application();

        void run();

        int getHttpPort() const;

        void setHttpPort(int port);

        int getShutPort() const;

        void setShutPort(int port);

        const std::string &getWebAppName() const;

        void setWebAppName(const std::string &name);

        const std::vector<std::string> &getSubApps() const;

        void setSubApps(const std::vector<std::string> &apps);

        bool isNewWindow() const;

        void setNewWindow(bool newWindow);
    };
}


#endif //TMS_APPLICATION_H
