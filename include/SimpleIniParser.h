#pragma once

#include <map>
#include <string>

std::string getProperty(const std::map<std::string, std::string> *node, const std::string &key, const std::string &defaultValue);

class SimpleIniParser {
public:
    typedef std::map<std::string, std::string> PropertyMap;
    typedef std::map<std::string, PropertyMap*> ResourceMap;

private:
    std::string _path;

    ResourceMap _resource;

    bool _valid;
    
    bool parse();

public:
    SimpleIniParser(const std::string &path);

    SimpleIniParser();

    ~SimpleIniParser();

    bool isValid() const;

    void load(const std::string &path);

    PropertyMap *getPropertyMap(const std::string &key) const;

    std::string get(const std::string &node, const std::string &key) const;

    std::string get(const std::string &node, const std::string &key, const std::string &defaultValue) const;
};