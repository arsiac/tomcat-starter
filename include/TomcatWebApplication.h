//
// Created by arsia on 2021/8/16.
//

#ifndef TMS_TOMCATWEBAPPLICATION_H
#define TMS_TOMCATWEBAPPLICATION_H
#include <string>
#include <boost/filesystem.hpp>
#include "tmsdef.h"
#include "SimpleLogger.h"
#include "WebApplication.h"
#include "AppConfig.h"

namespace tms {
    /**
     * class TomcatWebApplication
     */
    class TomcatWebApplication {
    private:
        typedef boost::filesystem::path path;

        Logger * log;

        path tomcat;
        path target;

        void removeAll(const path &dir);

    public:
        TomcatWebApplication(const path &_tomcat, const path &_target);

        /**
         * initialize tomcat configuration files
         *
         * @param _path application path
         * @param _http http port
         * @param _shut shutdown port
         */
        bool initializeConfig(int _http, int _shut);

        /**
         * set sub web application configuration file
         *
         * @param _context context path
         */
        bool createWebApplicationConfig(std::vector<WebApplication::SubWebApplication>  &apps);
        /**
         * start tomcat
         *
         * @param newWindow true: run in new window
         */
        void run(bool newWindow, const AppConfig *config);
    };
}


#endif //TMS_TOMCATWEBAPPLICATION_H
