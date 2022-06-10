#include "Tomcat.h"
#include "CommandLine.h"
#include "file.h"
#include "os.h"
#include "tmsdef.h"
#include <cstring>
#include <fstream>

Tomcat::Tomcat(const Project &project, const std::string &cacheBase, LogLevel level) {
    logger = new ConsoleLogger("Tomcat", level);
    _project = project;
    _cacheBase = cacheBase;
}

Tomcat::~Tomcat() { delete logger; }

void Tomcat::debugMode(bool enable) { _debugMode = enable; }

void Tomcat::otherWindow(bool enable) { _otherWindow = enable; }

bool Tomcat::check() { return checkJavaHome() && checkTomcat() && checkCache(); }

bool Tomcat::checkJavaHome() {
    logger->info("Java Home: " + _project.javaHome());

    // check java home
    if (fileNotExists(_project.javaHome())) {
        logger->error("java_home not exists: " + _project.javaHome());
        return false;
    }

    // check java
    std::string javaExe = _project.javaHome() + FILE_SEPARATOR + BIN_DIRECTORY + FILE_SEPARATOR + JAVA_EXECUTABLE;
    if (fileNotExists(javaExe)) {
        logger->error("the java_home configuration error, the '" + std::string(JAVA_EXECUTABLE) +
                      "' executable could not be found: " + javaExe);
        return false;
    }

    return true;
}

bool Tomcat::checkTomcat() {
    logger->info("Catalina Home: " + _project.tomcat());

    // check CATALINA_HOME
    if (fileNotExists(_project.tomcat())) {
        logger->error("tomcat not exists: " + _project.tomcat());
        return false;
    }

    // check catalina
    _catalinaExe = _project.tomcat() + FILE_SEPARATOR + BIN_DIRECTORY + FILE_SEPARATOR + CATALINA_EXECUTABLE;
    if (fileNotExists(_catalinaExe)) {
        logger->error("the tomcat configuration error, the '" + std::string(CATALINA_EXECUTABLE) +
                      "' executable could not be found: " + _catalinaExe);
        return false;
    }

    return true;
}

bool Tomcat::checkWebDocument(const std::string &name, const WebDocument &doc) {
    // check web document
    logger->info("check web document \"" + name + "\": " + doc.path());
    if (fileNotExists(doc.path())) {
        logger->error("web document \"" + name + "\" not exists: " + doc.path());
        return false;
    }
    return true;
}

bool Tomcat::checkCache() {
    logger->info("TMS Cache: " + _cacheBase);
    if (fileNotExists(_cacheBase)) {
        logger->error("cache_dir not exists: " + _cacheBase);
        return false;
    }

    std::string tmsCacheDir = _cacheBase + FILE_SEPARATOR + CACHE_DIRECTORY;
    if (!checkOrCreateDirectory(tmsCacheDir)) {
        return false;
    }

    _projectCache = tmsCacheDir + FILE_SEPARATOR + _project.name();
    if (!checkOrCreateDirectory(_projectCache)) {
        return false;
    }

    logger->info("Project Cache: " + _projectCache);
    _webappDir = _projectCache + FILE_SEPARATOR + WEBAPP_DIRECTORY;
    return true;
}

bool Tomcat::checkOrCreateDirectory(const std::string &path) {
    logger->debug("check directory: " + path);
    if (fileNotExists(path)) {
        logger->debug("create directory: " + path);
        if (!makeDirectory(path)) {
            logger->error("create directory failed: " + path);
            return false;
        }
    }

    return true;
}

bool Tomcat::copyTomcatFiles() {
    // copy configuration files
    std::string tomcatConfDir = _project.tomcat() + FILE_SEPARATOR + CONF_DIRECTORY;
    std::string cacheConfDir = _projectCache + FILE_SEPARATOR + CONF_DIRECTORY;
    logger->info("copy tomcat configuration files: " + tomcatConfDir);
    if (!copyDirectory(tomcatConfDir, cacheConfDir, false)) {
        return false;
    }
    return true;
}

bool Tomcat::createServerXml() {
    std::string serverXmlFile = _projectCache + FILE_SEPARATOR + CONF_DIRECTORY + FILE_SEPARATOR + "server.xml";
    std::string serverTemplate(TOMCAT_SERVER_TEMPLATE);

    std::string serverPort = "${server-port}";
    std::string httpPort = "${http-port}";

    serverTemplate.replace(serverTemplate.find(serverPort), serverPort.size(), _project.serverPort());
    serverTemplate.replace(serverTemplate.find(httpPort), httpPort.size(), _project.httpPort());

    logger->debug("generate server.xml: \n" + serverTemplate);

    // write server.xml
    logger->debug("write server.xml: " + serverXmlFile);
    std::ofstream out(serverXmlFile);
    if (!out.is_open()) {
        logger->error("cannot open file: " + serverXmlFile);
        return false;
    }
    out << serverTemplate;
    out.close();
    return true;
}

bool Tomcat::generateContextDirectory() {
    _contextDir = _projectCache + FILE_SEPARATOR + CONF_DIRECTORY + FILE_SEPARATOR + "Catalina";
    if (!checkOrCreateDirectory(_contextDir)) {
        return false;
    }

    _contextDir = _contextDir + FILE_SEPARATOR + "localhost";
    if (!checkOrCreateDirectory(_contextDir)) {
        return false;
    }
    return true;
}

bool Tomcat::cleanContextDirectory() {
    logger->info("clean context directory: " + _contextDir);
    if (fileNotExists(_contextDir)) {
        logger->warn("clean context directory failed: directory not exists: " + _contextDir);
        return true;
    }
    bool res = removeAllChildren(_contextDir);
    if (!res) {
        logger->error("clean context directory failed: " + _contextDir);
    }
    return res;
}

bool Tomcat::cleanWebappDirectory() { 
    logger->info("clean webapps directory: " + _webappDir);
    if (fileNotExists(_webappDir)) {
        logger->warn("clean context directory failed: directory not exists: " + _webappDir);
        return true;
    }
    bool res = removeAllChildren(_webappDir);
    if (!res) {
        logger->error("clean context directory failed: " + _webappDir);
    }
    return res;
}

bool Tomcat::createContext(const WebDocument &doc) {
    std::string contextName = doc.context().at(0) == '/' ? doc.context().substr(1) : doc.context();
    std::string contextXmlFile = _contextDir + FILE_SEPARATOR + contextName + ".xml";
    std::string contextTemplate(TOMCAT_CONTEXT_TEMPLATE);
    std::string context = "${context-path}";
    std::string docBase = "${web-document}";
    contextTemplate.replace(contextTemplate.find(context), context.size(), doc.context());
    contextTemplate.replace(contextTemplate.find(docBase), docBase.size(), doc.path());

    logger->debug("generate " + contextName + ".xml: \n" + contextTemplate);
    // write ${context}.xml
    logger->debug("write context XML: " + contextXmlFile);
    std::ofstream out(contextXmlFile);
    if (!out.is_open()) {
        logger->error("cannot open file: " + contextXmlFile);
        return false;
    }
    out << contextTemplate;
    out.close();
    return true;
}

void Tomcat::run(const std::list<std::string> &docList) {
    if (!_project.isPresent()) {
        logger->error("project " + _project.name() + " is not avaliable.");
        return;
    }

    logger->debug("tomcat run project: " + _project.name());
    if (!check()) {
        return;
    }

    if (!copyTomcatFiles()) {
        return;
    }

    if (!createServerXml()) {
        return;
    }

    if (!generateContextDirectory()) {
        return;
    }

    if (!cleanContextDirectory()) {
        return;
    }

    if (!cleanWebappDirectory()) {
        return;
    }

    auto webMap = _project.webMap();
    for (auto &item : docList) {
        auto pair = webMap.find(item);
        if (pair == webMap.end()) {
            logger->error("web document not exists: " + item);
            return;
        }

        if (!checkWebDocument(pair->first, pair->second)) {
            return;
        }

        if (!createContext(pair->second)) {
            logger->error("create context failed: " + item);
            return;
        }
    }

    CommandLine commandLine(_catalinaExe);
    commandLine.addEnvironment("JAVA_HOME", _project.javaHome());
    commandLine.addEnvironment("JRE_HOME", "");
    commandLine.addEnvironment("JAVA_OPTS", _project.javaOpts());
    commandLine.addEnvironment("CATALINA_HOME", _project.tomcat());
    commandLine.addEnvironment("CATALINA_BASE", _projectCache);
    if (_debugMode) {
        logger->debug("open Tomcat debug mode, using port " + _project.jpdaPort());
        commandLine.addEnvironment("JPDA_ADDRESS", "0.0.0.0:" + _project.jpdaPort());
        commandLine.addParamter("jpda");
    }
    commandLine.addParamter(_otherWindow ? "start" : "run");

    std::string command = commandLine.build();
    logger->debug("command: " + command);
    logger->info("The following is the log of Tomcat.");
    system(command.c_str());
}
