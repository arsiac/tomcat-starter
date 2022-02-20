#if defined(_WIN32) || defined(_WIN64)
#include "file.h"
#include "os.h"
#include <cstring>
#include <direct.h>
#include <io.h>

bool makeDirectory(const std::string &dir) { return 0 == _mkdir(dir.c_str()); }

bool removeEmptyDirectory(const std::string &dir) { return 0 == _rmdir(dir.c_str()); }

bool copyDirectory(const std::string &source, const std::string &target, bool recursion) {
    if (fileNotExists(source)) {
        return false;
    }

    if (fileNotExists(target) && !makeDirectory(target)) {
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

#endif // Windows