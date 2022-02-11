#pragma once

#include <string>
#include <list>

class CommandLine {
private:
    std::list<std::string> _environmentList;

    std::string _executable;

    std::list<std::string> _paramterList;

    public:
    CommandLine(const std::string &executable);

    void addEnvironment(const std::string &key, const std::string &value);

    void addParamter(const std::string &paramter);

    std::string build();
};