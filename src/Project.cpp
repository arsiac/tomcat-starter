#include "Project.h"

void WebDocument::context(const std::string &context) { _context = context; }

std::string WebDocument::context() const { return _context; }

void WebDocument::path(const std::string &path) { _path = path; }

std::string WebDocument::path() const { return _path; }

WebDocument::WebDocument(const std::string &context, const std::string &path) {
    _context = context.at(0) != '/' ? '/' + context : context;
    _path = path;
}

Project::Project() { _present = true; }

void Project::present(bool present) { _present = present; }

bool Project::isPresent() const { return _present; }

void Project::name(const std::string &name) { _name = name; }

std::string Project::name() const { return _name; }

void Project::javaHome(const std::string &javaHome) { _javaHome = javaHome; }

std::string Project::javaHome() const { return _javaHome; }

void Project::javaOpts(const std::string &javaOpts) { _javaOpts = javaOpts; }

std::string Project::javaOpts() const { return _javaOpts; }

void Project::tomcat(const std::string &tomcat) { _tomcat = tomcat; }

std::string Project::tomcat() const { return _tomcat; }

void Project::httpPort(const std::string &httpPort) { _httpPort = httpPort; }

std::string Project::httpPort() const { return _httpPort; }

void Project::serverPort(const std::string &serverPort) { _serverPort = serverPort; }

std::string Project::serverPort() const { return _serverPort; }

void Project::jpdaPort(const std::string &jpdaPort) { _jpdaPort = jpdaPort; }

std::string Project::jpdaPort() const { return _jpdaPort; }

void Project::webMap(const std::map<std::string, WebDocument> &map) { this->_webMap = map; }

std::map<std::string, WebDocument> Project::webMap() const { return _webMap; }
