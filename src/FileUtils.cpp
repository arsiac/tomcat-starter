//
// Created by arsia on 2021/8/18.
//

#include "FileUtils.h"

/**********************
 * FileUtils > Static *
 **********************/
const boost::filesystem::directory_iterator tms::FileUtils::end;

/**********************
 * FileUtils > Public *
 **********************/
bool tms::FileUtils::checkFile(const BoostPath &path) {
    if (!boost::filesystem::exists(path)) {
        return false;
    }

    if (boost::filesystem::is_directory(path)) {
        return false;
    }

    if (!boost::filesystem::is_regular_file(path)) {
        return false;
    }
    return true;
}

bool tms::FileUtils::checkDirectory(const BoostPath &path) {
    if (!boost::filesystem::exists(path)) {
        return false;
    }

    if (!boost::filesystem::is_directory(path)) {
        return false;
    }
    return true;
}

bool tms::FileUtils::remove(const tms::FileUtils::BoostPath &path) {
    std::string pathStr = path.string();
    if (!boost::filesystem::exists(path)) {
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

        if (!boost::filesystem::create_directories(dir)) {
            return false;
        }
    } else {
        FileUtils::removeAllSubItem(dir);
    }
    return true;
}

