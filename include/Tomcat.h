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

    std::string _catalinaExe;

    std::string _contextDir;

    bool _debugMode;

    bool _otherWindow;

    bool check();

    bool checkJavaHome();

    bool checkTomcat();

    bool checkWebDocument(const std::string &name, const WebDocument &doc);

    bool checkCache();

    bool checkOrCreateDirectory(const std::string &path);

    bool copyTomcatFiles();

    bool createServerXml();

    bool generateContextDirectory();

    bool cleanContextDirectory();

    bool createContext(const WebDocument &doc);

public:
    Tomcat(const Project &project, const std::string &cacheBase, LogLevel level);
    ~Tomcat();
    void debugMode(bool enable);
    void otherWindow(bool enable);
    void run(const std::list<std::string> &docList);
};