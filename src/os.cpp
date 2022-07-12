#include "os.h"

#if defined(_WIN32) || defined(_WIN64)
const char *OS_NAME = "windows";
const char FILE_SEPARATOR = '\\';
const char PATH_SEPARATOR = ';';
const char *S_FILE_SEPARATOR = "\\";
const char *S_PATH_SEPARATOR = ";";
const char *USER_HOME = "USERPROFILE";
const char *JAVA_EXECUTABLE = "java.exe";
const char *CATALINA_EXECUTABLE = "catalina.bat";
#else
const char *OS_NAME = "linux";
const char FILE_SEPARATOR = '/';
const char PATH_SEPARATOR = ':';
const char *S_FILE_SEPARATOR = "/";
const char *S_PATH_SEPARATOR = ":";
const char *USER_HOME = "HOME";
const char *JAVA_EXECUTABLE = "java";
const char *CATALINA_EXECUTABLE = "catalina.sh";
#endif