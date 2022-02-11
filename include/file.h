#pragma once

#include <string>

bool fileExists(const std::string &path);

bool fileNotExists(const std::string &path);

bool copyFile(const std::string &source, const std::string &target);

bool copyDirectory(const std::string &source, const std::string &target, bool recursion);

bool removeAllChildren(const std::string &dir);

bool removeDirectory(const std::string &dir);