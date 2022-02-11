#include "file.h"
#include "os.h"
#include <fstream>
#include <direct.h>
#include <sys/stat.h>
#include <io.h>


bool fileExists(const std::string &path) {
    struct stat buffer;
    return stat(path.c_str(), &buffer) == 0;
}

bool fileNotExists(const std::string &path) { return !fileExists(path); }

bool copyFile(const std::string &source, const std::string &target) {
    if (fileNotExists(source)) {
        return false;
    }

    std::ifstream in(source);
    std::ofstream out(target);

    if (!out.is_open()) {
        in.close();
        return false;
    }

    // copy
    out << in.rdbuf();

    // close stream
    in.close();
    out.close();
    return true;
}

bool copyDirectory(const std::string &source, const std::string &target, bool recursion) {
    if (fileNotExists(source)) {
        return false;
    }

    if (fileNotExists(target) && 0 != _mkdir(target.c_str())) {
        return false;
    }

    _finddata_t fileInfo;
    std::string pattern = source + FILE_SEPARATOR + '*';
    intptr_t handle = _findfirst(pattern.c_str(), &fileInfo);
    if (handle == -1) {
        return false;
    }

    do {
        std::string srcFile = source + FILE_SEPARATOR + fileInfo.name;
        std::string tgtFile = target + FILE_SEPARATOR + fileInfo.name;
        if (fileInfo.attrib & _A_SUBDIR) {
            if (recursion && 0 != strcmp(".", fileInfo.name) && 0 != strcmp("..", fileInfo.name)) {
                if (!copyDirectory(srcFile, tgtFile, true)) {
                    return false;
                }
            }
        } else {
            if (!copyFile(srcFile, tgtFile)) {
                return false;
            }
        }

    } while (0 == _findnext(handle, &fileInfo));

    _findclose(handle);
    return true;
}

bool removeAllChildren(const std::string &dir) {
    if (fileNotExists(dir)) {
        return false;
    }
    _finddata_t fileInfo;
    std::string pattern = dir + FILE_SEPARATOR + '*';
    intptr_t handle = _findfirst(pattern.c_str(), &fileInfo);
    if (handle == -1) {
        return false;
    }

    do {
        std::string file = dir + FILE_SEPARATOR + fileInfo.name;
        if (fileInfo.attrib & _A_SUBDIR) {
            if (0 == strcmp(".", fileInfo.name) || 0 == strcmp("..", fileInfo.name)) {
                continue;
            }
            if (!removeDirectory(file.c_str())) {
                return false;
            }
        } else {
            if (0 != remove(file.c_str())) {
                return false;
            }
        }

    } while (0 == _findnext(handle, &fileInfo));

    _findclose(handle);
    return true;
}

bool removeDirectory(const std::string &dir) { 
    if (removeAllChildren(dir)) {
        return 0 == _rmdir(dir.c_str());
    }

    return false;
}