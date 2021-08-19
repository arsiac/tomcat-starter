//
// Created by arsia on 2021/8/15.
//

#ifndef TMS_TMSDEF_H
#define TMS_TMSDEF_H
#include <string>

extern const std::string CG_GLOBAL_HTTP;
extern const std::string CG_GLOBAL_SHUT;
extern const std::string CG_GLOBAL_JVM_OPTS;

extern const std::string CG_GROUP_COMMON;
extern const std::string CG_COMMON_LOG_LEVEL;
extern const std::string CG_COMMON_JAVA_HOME;
extern const std::string CG_COMMON_TOMCAT;

extern const std::string CG_WEB_CONTEXT_SUFFIX;
extern const std::string CG_WEB_WAR_SUFFIX;

extern const std::string ENV_JAVA_HOME;
extern const std::string ENV_JAVA_OPTS;

extern const char *CG_TEMPLATE;

extern const char *TOMCAT_SERVER_TEMPLATE;

extern const char *TOMCAT_APP_TEMPLATE;

extern const char *TOMCAT_ENV_CMD;

extern const char *TOMCAT_CATALINA_CMD;

#endif //TMS_TMSDEF_H
