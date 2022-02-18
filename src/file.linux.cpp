#if defined(__linux__) || defined(__unix__)
#include "file.h"
#include "os.h"
#include <cstring>
#include <unistd.h>
#include <dirent.h>
#include <sys/stat.h>

__mode_t FILE_MODE = S_IRWXU | S_IRGRP | S_IXGRP | S_IROTH | S_IXOTH;

bool makeDirectory(const std::string &dir) { return 0 == mkdir(dir.c_str(), FILE_MODE); }

bool removeEmptyDirectory(const std::string &dir) { return 0 == rmdir(dir.c_str()); }

bool copyDirectory(const std::string &source, const std::string &target, bool recursion) {
    if (fileNotExists(source)) {
        return false;
    }

    if (fileNotExists(target) && 0 != makeDirectory(target)) {
        return false;
    }

    DIR *dp = opendir(source.c_str());
    if (!dp) { 
        return false;
    }
    
    dirent *file;
    while(file = readdir(dp)) {
        std::string srcFile = source + FILE_SEPARATOR + file->d_name;
        std::string tgtFile = target + FILE_SEPARATOR + file->d_name;

        if (file->d_type == DT_DIR ) {
            if (recursion && 0 != strcmp(".", file->d_name) && 0 != strcmp("..", file->d_name)) {
                if (!copyDirectory(srcFile, tgtFile, true)) {
                    return false;
                }
            }
        } else if (file->d_type == DT_REG) {
            if (!copyFile(srcFile, tgtFile)) {
                return false;
            }
        }
    }

    closedir(dp);
    return true;
}

bool removeAllChildren(const std::string &dir) {
    if (fileNotExists(dir)) {
        return false;
    }
    
    DIR *dp = opendir(dir.c_str());
    if (!dp) { 
        return false;
    }
    
    dirent *file;
    std::string pattern = dir + FILE_SEPARATOR + '*';

    while(file = readdir(dp)) {
        std::string fileName = dir + FILE_SEPARATOR + file->d_name;
        if (file->d_type == DT_DIR) {
            if (0 == strcmp(".", file->d_name) || 0 == strcmp("..", file->d_name)) {
                continue;
            }
            if (!removeDirectory(fileName.c_str())) {
                return false;
            }
        } else {
            if (0 != remove(fileName.c_str())) {
                return false;
            }
        }
    }

    closedir(dp);
    return true;
}
#endif