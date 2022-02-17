#include "TomcatStarter.h"

bool clean(const std::string &dir);

int main(int argc, const char *argv[]) {
    Argument args(argc, argv);
    if (!args.isValid()) {
        return 1;
    }

    Configuration configuration;
    if (args.isClean()) {
        return clean(configuration.baseCache() + FILE_SEPARATOR + CACHE_DIRECTORY) ? 0 : 1;
    }

    Project project = configuration.getProject(args.projectName());
    if (!args.httpPort().empty()) {
        project.httpPort(args.httpPort());
    }
    if (!args.serverPort().empty()) {
        project.serverPort(args.serverPort());
    }
    if (!args.jpdaPort().empty()) {
        project.jpdaPort(args.jpdaPort());
    }

    Tomcat tomcat(project, configuration.baseCache(), configuration.logLevel());
    tomcat.debugMode(args.debugMode());
    tomcat.otherWindow(args.otherWindow());
    tomcat.run(args.docList());
    return 0;
}

bool clean(const std::string& dir) {
    if (fileNotExists(dir)) {
        return true;
    }
    bool res = removeAllChildren(dir);
    if (!res) {
        std::cerr << "error: clean cache directory failed: " << dir << std::endl;
    }
    return res;
}