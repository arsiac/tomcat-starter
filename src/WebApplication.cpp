//
// Created by arsia on 2021/8/14.
//

#include "WebApplication.h"


/*******************************************************
 * WebApplication::SubWebApplication > Getter & Setter *
 *******************************************************/
const std::string &tms::WebApplication::SubWebApplication::getName() const {
    return name;
}

void tms::WebApplication::SubWebApplication::setName(const std::string &_name) {
    SubWebApplication::name = _name;
}

const std::string &tms::WebApplication::SubWebApplication::getContext() const {
    return context;
}

void tms::WebApplication::SubWebApplication::setContext(const std::string &_context) {
    SubWebApplication::context = _context;
}

const boost::filesystem::path &tms::WebApplication::SubWebApplication::getWar() const {
    return war;
}

void tms::WebApplication::SubWebApplication::setWar(const boost::filesystem::path &_war) {
    SubWebApplication::war = _war;
}

/********************************
 * WebApplication > Constructor *
 ********************************/
tms::WebApplication::~WebApplication() {
    for (auto & appItr : subAppMap) {
        delete appItr.second;
    }
}

/***************************
 * WebApplication > Public *
 ***************************/
tms::WebApplication::SubWebApplication *tms::WebApplication::getOrCreateSubApp(const std::string &_name) {
    if (subAppMap.count(_name)) {
        return subAppMap.find(_name)->second;
    }
    auto app = new SubWebApplication();
    app->setName(_name);
    subAppMap.insert(std::make_pair(_name, app));
    return app;
}

void tms::WebApplication::deleteSubApp(const std::string &_name) {
    if (subAppMap.count(_name)) {
        auto app = subAppMap.find(_name)->second;
        subAppMap.erase(_name);
        delete app;
    }
}

bool tms::WebApplication::isEmpty() {
    return subAppMap.empty();
}

bool tms::WebApplication::contains(const std::string &subName) {
    return subAppMap.count(subName);
}

/************************************
 * WebApplication > Getter & Setter *
 ************************************/
const std::string &tms::WebApplication::getName() const {
    return name;
}

void tms::WebApplication::setName(const std::string &_name) {
    WebApplication::name = _name;
}

int tms::WebApplication::getHttpPort() const {
    return httpPort;
}

void tms::WebApplication::setHttpPort(int port) {
    WebApplication::httpPort = port;
}

int tms::WebApplication::getShutPort() const {
    return shutPort;
}

void tms::WebApplication::setShutPort(int port) {
    WebApplication::shutPort = port;
}

const std::string &tms::WebApplication::getJvmOptions() const {
    return jvmOptions;
}

void tms::WebApplication::setJvmOptions(const std::string &options) {
    WebApplication::jvmOptions = options;
}
