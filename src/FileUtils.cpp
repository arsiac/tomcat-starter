//
// Created by arsia on 2021/8/18.
//

#include "FileUtils.h"

/**********************
 * FileUtils > Static *
 **********************/
const boost::filesystem::directory_iterator tms::FileUtils::end;
tms::Logger *tms::FileUtils::log = tms::SimpleLogFactory::getGlobal(); /* NOLINT */

/**********************
 * FileUtils > Public *
 **********************/
bool tms::FileUtils::checkFile(const BoostPath &path) {
    if (!boost::filesystem::exists(path)) {
        log->error() << "file not exist: " << path.string() << std::endl;
        return false;
    }

    if (boost::filesystem::is_directory(path)) {
        log->error() << "it's not a file: " << path.string() << std::endl;
        return false;
    }

    if (!boost::filesystem::is_regular_file(path)) {
        log->error() << "it's not a regular file: " << path.string() << std::endl;
        return false;
    }
    return true;
}

bool tms::FileUtils::checkDirectory(const BoostPath &path) {
    if (!boost::filesystem::exists(path)) {
        log->error() << "directory not exist: " << path.string() << std::endl;
        return false;
    }

    if (!boost::filesystem::is_directory(path)) {
        log->error() << "it's not a directory: " << path.string() << std::endl;
        return false;
    }
    return true;
}

bool tms::FileUtils::remove(const tms::FileUtils::BoostPath &path) {
    std::string pathStr = path.string();
    log->debug() << "remove: " << pathStr << std::endl;
    if (!boost::filesystem::exists(path)) {
        log->error() << "remove file or directory failed. file not exist: " << pathStr << std::endl;
        return false;
    }

    if (!boost::filesystem::is_directory(path)) {
        boost::filesystem::remove_all(path);
    } else {
        boost::filesystem::remove(path);
    }
    return true;
}

bool tms::FileUtils::removeAllSubItem(const tms::FileUtils::BoostPath &dir) {
    std::string pathStr = dir.string();
    log->debug() << "remove: " << pathStr
                 #if defined(WIN32) || defined(WIN64)
                 << "\\*"
                 #else
                 << "/*"
                 #endif
                 << std::endl;
    if (!checkDirectory(dir)) {
        return false;
    }

    for (BoostDirIterator sub(dir); sub != end; sub++) {
        BoostPath path = sub->path();
        if (boost::filesystem::is_directory(path)) {
            boost::filesystem::remove_all(path);
        } else {
            boost::filesystem::remove(path);
        }
    }
    return true;
}

bool tms::FileUtils::createOrCleanDir(const BoostPath &dir) {
    std::string pathStr = dir.string();
    if (!boost::filesystem::exists(dir)) {
        log->debug() << "directory not exist, try create directory: " << pathStr << std::endl;

        if (!boost::filesystem::create_directories(dir)) {
            log->error() << "create directory failed: " << pathStr << std::endl;
            return false;
        }
    } else {
        log->debug() << "clean directory: " << pathStr << std::endl;
        FileUtils::removeAllSubItem(dir);
    }
    return true;
}

