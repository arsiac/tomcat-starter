#include "file.h"
#include <fstream>
#include <sys/stat.h>

bool fileExists(const std::string &path) {
    struct stat buffer;
    return stat(path.c_str(), &buffer) == 0;
}

bool fileNotExists(const std::string &path) { return !fileExists(path); }

bool copyFile(const std::string &source, const std::string &target) {
    std::ifstream in(source);
    if (!in) {
        return false;
    }
    std::ofstream out(target);

    bool result = false;
    if (out) {
        out << in.rdbuf();
        out.close();
        result = true;
    }

    in.close();
    return result;
}

bool removeDirectory(const std::string &dir) { 
    if (removeAllChildren(dir)) {
        return removeEmptyDirectory(dir);
    }

    return false;
}