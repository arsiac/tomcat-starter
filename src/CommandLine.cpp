#include "CommandLine.h"

CommandLine::CommandLine(const std::string &executable) { _executable = executable; }

void CommandLine::addEnvironment(const std::string &key, const std::string &value) {
    std::string env = value.empty() ? key + "=" : key + "=" + value;
#if defined(WIN32) || defined(WIN64)
    _environmentList.push_back("set " + env);
#else
    _environmentList.push_back(env);
#endif
}

void CommandLine::addParamter(const std::string &paramter) { _paramterList.push_back(paramter); }

std::string CommandLine::build() { 
    const std::string divide = "&&";
    const std::string space = " ";
    std::string command;
    for (auto &item : _environmentList) {
        command.append(item).append(divide);
    }
    command.append(_executable);
    for (auto &item : _paramterList) {
        command.append(space).append(item);
    }
    return command;
}
