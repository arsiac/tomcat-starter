//
// Created by arsia on 2021/8/16.
//

#include "TomcatWebApplication.h"
#include "StringUtils.h"
#include "FileUtils.h"

static const char *VARIABLE_HTTP_PORT = "${http-port}";
static const char *VARIABLE_SHUTDOWN_PORT = "${shut-port}";
static const char *VARIABLE_CONTEXT_PATH = "${context-path}";
static const char *VARIABLE_APP_PATH = "${app-path}";
static const char *VARIABLE_JAVA_HOME = "${java-home}";
static const char *VARIABLE_JVM_OPTS = "${jvm-opts}";
static const char *VARIABLE_CATALINA_HOME = "${catalina-home}";
static const char *VARIABLE_CATALINA_BASE = "${catalina-base}";

/**************************************
 * TomcatWebApplication > Constructor *
 **************************************/
tms::TomcatWebApplication::TomcatWebApplication(const path &_tomcat, const path &_target) {
    log = SimpleLogFactory::getLogger("TomcatWebApplication");
    tomcat = _tomcat;
    target = _target;
    log->info() << "tomcat location: " << _tomcat.string() << std::endl;
    log->info() << "target location: " << _target.string() << std::endl;
}

/**********************************
 * TomcatWebApplication > Private *
 **********************************/
void tms::TomcatWebApplication::removeAll(const path &dir) {
    boost::filesystem::directory_iterator end;
    for (boost::filesystem::directory_iterator fileItr(dir); fileItr != end; fileItr++) {
        boost::filesystem::path filePath = fileItr->path();
        if (boost::filesystem::is_directory(filePath)) {
            boost::filesystem::remove_all(filePath);
        } else {
            boost::filesystem::remove(filePath);
        }
        log->debug() << "remove: " << filePath.generic_string() << std::endl;
    }
}

/*********************************
 * TomcatWebApplication > Public *
 *********************************/
bool tms::TomcatWebApplication::initializeConfig(int _http, int _shut) {
    path src = tomcat;
    src.append("conf");

    if (!boost::filesystem::exists(src)) {
        log->error() << "cannot found tomcat config directory: " << src.string() << std::endl;
        return false;
    }

    path conf = target;
    conf.append("conf");
    if (!boost::filesystem::exists(conf)) {
        log->debug() << "directory not exist, try create directory: " << conf.string() << std::endl;

        if (!boost::filesystem::create_directories(conf)) {
            log->error() << "create directory failed: " << conf.string() << std::endl;
            return false;
        }
    }

    boost::filesystem::directory_iterator dirEnd;
    for (boost::filesystem::directory_iterator fileItr(src); fileItr != dirEnd; fileItr++) {
        path filePath = fileItr->path();
        std::string fileName = filePath.filename().string();

        if (fileName != "server.xml" && boost::filesystem::is_regular_file(filePath)) {
            path targetFilePath = conf;
            targetFilePath.append(fileName);
            boost::filesystem::copy_file(filePath, targetFilePath, boost::filesystem::copy_option::overwrite_if_exists);
            log->debug() << "copy file: " << targetFilePath.string() << std::endl;
        }
    }

    // generate server.xml
    std::string serverXmlContent(TOMCAT_SERVER_TEMPLATE);
    StringUtils::replaceAll(serverXmlContent, VARIABLE_HTTP_PORT, std::to_string(_http));
    StringUtils::replaceAll(serverXmlContent, VARIABLE_SHUTDOWN_PORT, std::to_string(_shut));

    log->debug("generate server.xml") << serverXmlContent << std::endl;

    path serverXml = conf;
    serverXml.append("server.xml");

    boost::filesystem::ofstream serverXmlFile(serverXml.string());
    serverXmlFile << serverXmlContent;
    serverXmlFile.close();
    return true;
}

bool tms::TomcatWebApplication::createWebApplicationConfig(std::vector<WebApplication::SubWebApplication> &apps) {
    path appConfigPath = target;
    appConfigPath.append("conf").append("Catalina").append("localhost");

    path webAppsDirPath = target;
    webAppsDirPath.append("webapps");

    boost::filesystem::directory_iterator dirEnd;

    if (!FileUtils::createOrCleanDir(webAppsDirPath)) {
        return false;
    }

    if (!FileUtils::createOrCleanDir(appConfigPath)) {
        return false;
    }

    // add context config
    boost::filesystem::ofstream file;
    for (auto &app : apps) {
        path targetAppPath = webAppsDirPath;
        targetAppPath.append(app.getContext() + ".war");
        path storePath = appConfigPath;
        storePath.append(app.getContext() + ".xml");

        std::string appContext(TOMCAT_APP_TEMPLATE);
        StringUtils::replaceAll(appContext, VARIABLE_CONTEXT_PATH, app.getContext());
        StringUtils::replaceAll(appContext, VARIABLE_APP_PATH, app.getWar().string());

        // write config file
        log->debug(app.getName() + " context xml configuration:") << appContext << std::endl;
        file.open(storePath.string());
        file << appContext;
        file.close();
    }
    return true;
}

void tms::TomcatWebApplication::run(bool newWindow, const AppConfig *config) {
    std::string catalina = tomcat.string();
    catalina.append(TOMCAT_CATALINA_CMD);

    const WebApplication &webApplication = config->getWebApplication();

    if (newWindow) {
        catalina.append(" start");
    } else {
        catalina.append(" run");
    }

    std::string cmd(TOMCAT_ENV_CMD);
    StringUtils::replaceAll(cmd, VARIABLE_JAVA_HOME, config->getCommonJavaHome().string());

    // generate JVM Options
    std::string jvmOpts = config->getCommonJvmOptions();
    if (!webApplication.getJvmOptions().empty()) {
        if (!jvmOpts.empty()) {
            jvmOpts.append(" ");
        }
        jvmOpts.append(webApplication.getJvmOptions());
    }
    StringUtils::replaceAll(cmd, VARIABLE_JVM_OPTS, jvmOpts);
    StringUtils::replaceAll(cmd, VARIABLE_CATALINA_HOME, tomcat.string());
    StringUtils::replaceAll(cmd, VARIABLE_CATALINA_BASE, target.string());
    cmd.append("&").append(catalina);

    log->debug() << cmd << std::endl;
    system(cmd.c_str());
}


