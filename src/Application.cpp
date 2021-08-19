//
// Created by arsia on 2021/8/11.
//

#include "Application.h"

/*****************************
 * Application > Constructor *
 *****************************/
tms::Application::Application(AppConfig *_config) {
    log = SimpleLogFactory::getLogger("Application");
    httpPort = -1;
    shutPort = -1;
    config = _config;
}

tms::Application::~Application() {
    delete config;
}

/************************
 * Application > Public *
 ************************/
void tms::Application::run() {
    log->info() << "Web Application: " << webAppName << std::endl;

    if (!config->load(webAppName)) {
        return;
    }

    WebApplication webApplication = config->getWebApplication();

    std::vector<WebApplication::SubWebApplication> subAppCache;
    // check sub app is exists
    for (const auto &app : subApps) {
        if (!webApplication.contains(app)) {
            log->error() << "cannot found `" << app << "` under `"
            << webApplication.getName() << "`." << std::endl;
            return;
        } else {
            subAppCache.push_back(*webApplication.getOrCreateSubApp(app));
        }
    }

    // set port
    if (httpPort == -1) {
        httpPort = webApplication.getHttpPort();
    }

    if (shutPort == -1) {
        shutPort = webApplication.getShutPort();
    }

    path pgmPath(_pgmptr);

    TomcatWebApplication tomcatWebApplication(config->getCommonTomcatLocation(),
                                              pgmPath.branch_path().append("tms-web").append(webAppName));

    if (!tomcatWebApplication.initializeConfig(httpPort, shutPort)) {
        log->error("initialize tomcat configuration failed");
        return;
    }

    if (!tomcatWebApplication.createWebApplicationConfig(subAppCache)) {
        log->error("create tomcat web application failed");
        return;
    }

    tomcatWebApplication.run(isNewWindow(), config);
}

/*********************************
 * Application > Getter & Setter *
 *********************************/
int tms::Application::getHttpPort() const {
    return httpPort;
}

void tms::Application::setHttpPort(int port) {
    Application::httpPort = port;
}

int tms::Application::getShutPort() const {
    return shutPort;
}

void tms::Application::setShutPort(int port) {
    Application::shutPort = port;
}

const std::string &tms::Application::getWebAppName() const {
    return webAppName;
}

void tms::Application::setWebAppName(const std::string &name) {
    Application::webAppName = name;
}

const std::vector<std::string> &tms::Application::getSubApps() const {
    return subApps;
}

void tms::Application::setSubApps(const std::vector<std::string> &apps) {
    Application::subApps = apps;
}

bool tms::Application::isNewWindow() const {
    return newWindow;
}

void tms::Application::setNewWindow(bool _new) {
    Application::newWindow = _new;
}

