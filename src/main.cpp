
#include "SimpleLogger.h"
#include "Application.h"
#include <vector>
#include <string>
#include <boost/program_options.hpp>

static tms::Logger *LOG;

bool parse_cli(int argc, const char *argv[], boost::program_options::variables_map &variablesMap) {
    boost::program_options::options_description description("Usage");
    description.add_options()
            ("app,a", boost::program_options::value<std::string>(), "web Application")
            ("sub-app,s", boost::program_options::value<std::vector<std::string>>() -> multitoken(), "sub item of web Application")
            ("http-port,l", boost::program_options::value<int>(), "http listen port")
            ("shut-port,t", boost::program_options::value<int>(), "shutdown listen port")
            ("config,c", boost::program_options::value<std::string>(), "configuration file path")
            ("new-window,n", "open in new window")
            ("help,h", "show options")
            ("config-template", "configuration file template");

    boost::program_options::command_line_parser parser(argc,argv);
    parser.options(description);
    try {
        boost::program_options::store(parser.run(), variablesMap);
    }
    catch (boost::program_options::error_with_no_option_name &ex) {
        LOG->error(ex.what());
        return false;
    }

    if (variablesMap.count("help") || variablesMap.empty()) {
        std::cout << description << std::endl;
        return false;
    }

    if (variablesMap.count("config-template")) {
        std::cout << CG_TEMPLATE << std::endl;
        return false;
    }

    return true;
}

bool initApp(tms::Application &app, boost::program_options::variables_map &map) {
    if (map.count("http-port") > 0) {
        app.setHttpPort(map["http-port"].as<int>());
    }

    if (map.count("shut-port") > 0) {
        app.setShutPort(map["shut-port"].as<int>());
    }

    app.setNewWindow(map.count("new-window"));

    if (map.count("app") > 0) {
        app.setWebAppName(map["app"].as<std::string>());
    } else {
        LOG->error("option 'app' is required.");
        return false;
    }

    if (map.count("sub-app") > 0) {
        app.setSubApps(map["sub-app"].as<std::vector<std::string>>());
    } else {
        LOG->error("option 'sub-app' is required.");
        return false;
    }

    return true;
}

int main(int argc, const char *argv[]) {
    // default log level WARN
    tms::SimpleLogFactory::setTimeFormat("%Y-%m-%d %H:%M:%S.%f");
    tms::SimpleLogFactory::setLogLevel(tms::LogLevel::WARN);
    LOG = tms::SimpleLogFactory::getLogger("Main");

    boost::program_options::variables_map option_map;

    // parse cli arguments
    if (!parse_cli(argc, argv, option_map)) {
        return 1;
    }

    // initialize config
    tms::AppConfig *config;
    if (option_map.count("config") > 0) {
        config = new tms::AppConfig(option_map["config"].as<std::string>());
    } else {
        config = new tms::AppConfig(boost::filesystem::path(_pgmptr).parent_path().append("config.ini"));
    }

    // initialize global log level
    tms::SimpleLogFactory::setLogLevel(config->getCommonLogLevel());

    // initialize app
    tms::Application app(config);
    if (!initApp(app, option_map)) {
        LOG->error("initialize Application failed.");
        return 1;
    }

    app.run();
    return 0;
}
