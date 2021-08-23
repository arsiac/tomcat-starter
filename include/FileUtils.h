//
// Created by arsia on 2021/8/18.
//

#ifndef TMS_FILEUTILS_H
#define TMS_FILEUTILS_H

#include <boost/filesystem.hpp>
#include "SimpleLogger.h"

namespace tms {
    class FileUtils {
        typedef boost::filesystem::path BoostPath;
        typedef boost::filesystem::directory_iterator BoostDirIterator;
    private:
        static const BoostDirIterator end;

    public:
        bool static checkFile(const BoostPath &path);

        bool static checkDirectory(const BoostPath &path);

        bool static remove(const BoostPath &path);

        bool static removeAllSubItem(const BoostPath &dir);

        bool static createOrCleanDir(const BoostPath &dir);
    };
}


#endif //TMS_FILEUTILS_H
