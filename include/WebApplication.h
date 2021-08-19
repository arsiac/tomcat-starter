//
// Created by arsia on 2021/8/14.
//

#ifndef TMS_WEBAPPLICATION_H
#define TMS_WEBAPPLICATION_H

#include <string>
#include <map>
#include <boost/filesystem.hpp>

namespace tms {
    /**
     * class WebApplication
     */
    class WebApplication {
    public:
        class SubWebApplication;

    private:
        std::string name;
        int httpPort;
        int shutPort;
        std::string jvmOptions;
        std::map<std::string,SubWebApplication*> subAppMap;

    public:
        ~WebApplication();

        SubWebApplication *getOrCreateSubApp(const std::string &_name);

        void deleteSubApp(const std::string &_name);

        bool isEmpty();

        bool contains(const std::string &subName);

        const std::string &getName() const;

        void setName(const std::string &_name);

        int getHttpPort() const;

        void setHttpPort(int port);

        int getShutPort() const;

        void setShutPort(int port);

        const std::string &getJvmOptions() const;

        void setJvmOptions(const std::string &options);
    };

    /**
     * class WebApplication::SubWebApplication
     */
    class WebApplication::SubWebApplication {
    private:
        std::string name;
        std::string context;
        boost::filesystem::path war;

    public:
        const std::string &getName() const;

        void setName(const std::string &_name);

        const std::string &getContext() const;

        void setContext(const std::string &_context);

        const boost::filesystem::path &getWar() const;

        void setWar(const boost::filesystem::path &_war);
    };
}


#endif //TMS_WEBAPPLICATION_H
