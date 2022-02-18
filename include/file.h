#pragma once

#include <string>

extern bool fileExists(const std::string &path);

extern bool fileNotExists(const std::string &path);

extern bool copyFile(const std::string &source, const std::string &target);

extern bool copyDirectory(const std::string &source, const std::string &target, bool recursion);

extern bool removeAllChildren(const std::string &dir);

extern bool removeDirectory(const std::string &dir);

extern bool makeDirectory(const std::string &dir);

extern bool removeEmptyDirectory(const std::string &dir);