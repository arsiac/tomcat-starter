#include "os.h"

#ifdef WIN32
const char FILE_SEPARATOR = '\\';
const char PATH_SEPARATOR = ';';
const char *S_FILE_SEPARATOR = "\\";
const char *S_PATH_SEPARATOR = ";";
const char *USER_HOME = "USERPROFILE";
const char *JAVA_EXECUTABLE = "java.exe";
const char *CATALINA_EXECUTABLE = "catalina.bat";
#else
const char FILE_SEPARATOR = '/';
const char PATH_SEPARATOR = ':';
const char *S_FILE_SEPARATOR = "/";
const char *S_PATH_SEPARATOR = ":";
const char *USER_HOME = "HOME";
const char *JAVA_EXECUTABLE = "java";
const char *CATALINA_EXECUTABLE = "catalina.sh";
#endif