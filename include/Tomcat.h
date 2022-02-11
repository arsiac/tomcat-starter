#pragma once

#include "Project.h"
#include "ConsoleLogger.h"
#include <string>
#include <list>

class Tomcat {
private:
    Logger *logger;

    Project _project;

    std::string _cacheBase;

    std::string _projectCache;

    std::string _javaExe;

    std::string _catalinaExe;

    std::string _contextDir;

    bool check();

    bool checkJavaHome();

    bool checkTomcat();

    bool checkWebDocument();

    bool checkCache();

    bool checkOrCreateDirectory(const std::string &path);

    bool copyTomcatFiles();

    bool createServerXml();

    bool generateContextDirectory();

    bool cleanContextDirectory();

    bool createContext(WebDocument doc);

public:
    Tomcat(const Project &project, const std::string &cacheBase, LogLevel level);
    ~Tomcat();
    void run(std::list<std::string> docList, bool enableDebugMode, bool openInNewWindow);
};