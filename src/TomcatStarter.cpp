#include "TomcatStarter.h"

bool clean(const std::string &dir);

int main(int argc, const char *argv[]) {
    Argument args(argc, argv);
    if (!args.isValid()) {
        return 1;
    }

    Configuration configuration;
    if (args.isClean()) {
        return clean(configuration.getCache() + FILE_SEPARATOR + "tms_cache") ? 0 : 1;
    }

    Project project = configuration.getProject(args.getProject());
    Tomcat tomcat(project, configuration.getCache(), configuration.getLogLevel());
    tomcat.run(args.getDocList(), args.isDebugMode(), args.isNewWindow());
    return 0;
}

bool clean(const std::string& dir) {
    bool res = removeAllChildren(dir);
    if (!res) {
        std::cerr << "error: clean cache directory failed: " << dir << std::endl;
    }
    return res;
}