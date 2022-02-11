#pragma once

#include <map>
#include <string>

class WebDocument {
private:
    std::string _context;

    std::string _path;

public:
    WebDocument(const std::string &context, const std::string &path);

    void context(const std::string &context);

    std::string context() const;

    void path(const std::string &path);

    std::string path() const;
};

class Project {
private:
    bool _present;

    std::string _name;

    std::string _javaHome;

    std::string _javaOpts;

    std::string _tomcat;

    std::string _httpPort;

    std::string _serverPort;

    std::string _jpdaPort;

    std::map<std::string, WebDocument> _webMap;

public:
    Project();

    void present(bool present);

    bool isPresent() const;

    void name(const std::string &name);

    std::string name() const;

    void javaHome(const std::string &javaHome);

    std::string javaHome() const;

    void javaOpts(const std::string &javaOpts);

    std::string javaOpts() const;

    void tomcat(const std::string &tomcat);

    std::string tomcat() const;

    void httpPort(const std::string &httpPort);

    std::string httpPort() const;

    void serverPort(const std::string &httpPort);

    std::string serverPort() const;

    void jpdaPort(const std::string &httpPort);

    std::string jpdaPort() const;

    void webMap(const std::map<std::string, WebDocument> &map);

    std::map<std::string, WebDocument> webMap() const;
};