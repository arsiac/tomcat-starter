//
// Created by arsia on 2021/8/15.
//
#include "tmsdef.h"

const std::string CG_GLOBAL_HTTP = "http_port"; /* NOLINT */
const std::string CG_GLOBAL_SHUT = "shut_port"; /* NOLINT */
const std::string CG_GLOBAL_JVM_OPTS = "jvm_opts"; /* NOLINT */

const std::string CG_GROUP_COMMON = "common"; /* NOLINT */
const std::string CG_COMMON_LOG_LEVEL = "log_level"; /* NOLINT */
const std::string CG_COMMON_JAVA_HOME = "java_home"; /* NOLINT */
const std::string CG_COMMON_TOMCAT = "tomcat"; /* NOLINT */

const std::string CG_WEB_CONTEXT_SUFFIX = "_context"; /* NOLINT */
const std::string CG_WEB_WAR_SUFFIX = "_war"; /* NOLINT */

const std::string ENV_JAVA_HOME = "JAVA_HOME"; /* NOLINT */
const std::string ENV_JAVA_OPTS = "JAVA_OPTS"; /* NOLINT */

const char *CG_TEMPLATE = "[common]\n"
                          "log_level = info\n"
                          "java_home = /path/to/jdk/\n"
                          ";global JVM options\njvm_opts = -XX:+HeapDumpOnOutOfMemoryError -XX:-OmitStackTraceInFastThrow -Xmx2048m -Dfile.encoding=UTF-8\n"
                          "tomcat = /path/to/catalina/home\n"
                          ";default http port\nhttp_port = 8080\n"
                          ";default shutdown port\nshut_port = 8079\n\n"
                          ";web application name\n[web-tst]\n"
                          ";web application http port.\n;use default http port if not set\n;http_port = 8082\n"
                          ";web application shutdown port.\n;use default shutdown port if not set\n;shut_port = 8081\n"
                          ";JVM option for this web application\n;jvm_opts = -Dapp=App\n"
                          ";sub web application context path\nanyname_context = app-web\n"
                          ";sub web application war path\nanyname_war = /pat/to/war\n"
                          ";anyname2_context = app2-web\n"
                          ";anyname2_war = /pat/to/war\n";

const char *TOMCAT_SERVER_TEMPLATE = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n"
                                     "<Server port=\"${shut-port}\" shutdown=\"SHUTDOWN\">\n"
                                     "  <Listener className=\"org.apache.catalina.startup.VersionLoggerListener\" />\n"
                                     "  <Listener className=\"org.apache.catalina.core.AprLifecycleListener\" SSLEngine=\"on\" />\n"
                                     "  <Listener className=\"org.apache.catalina.core.JreMemoryLeakPreventionListener\" />\n"
                                     "  <Listener className=\"org.apache.catalina.mbeans.GlobalResourcesLifecycleListener\" />\n"
                                     "  <Listener className=\"org.apache.catalina.core.ThreadLocalLeakPreventionListener\" />\n"
                                     "  <GlobalNamingResources>\n"
                                     "    <Resource name=\"UserDatabase\" auth=\"Container\" type=\"org.apache.catalina.UserDatabase\" description=\"User database that can be updated and saved\" factory=\"org.apache.catalina.users.MemoryUserDatabaseFactory\" pathname=\"conf/tomcat-users.xml\" />\n"
                                     "  </GlobalNamingResources>\n"
                                     "  <Service name=\"Catalina\">\n"
                                     "    <Connector port=\"${http-port}\" protocol=\"HTTP/1.1\"\n"
                                     "               connectionTimeout=\"20000\"\n"
                                     "               redirectPort=\"8443\" />\n"
                                     "    <Engine name=\"Catalina\" defaultHost=\"localhost\">\n"
                                     "      <Realm className=\"org.apache.catalina.realm.LockOutRealm\">\n"
                                     "        <Realm className=\"org.apache.catalina.realm.UserDatabaseRealm\" resourceName=\"UserDatabase\"/>\n"
                                     "      </Realm>\n"
                                     "      <Host name=\"localhost\"  appBase=\"webapps\" unpackWARs=\"true\" autoDeploy=\"true\">\n"
                                     "        <Valve className=\"org.apache.catalina.valves.AccessLogValve\" directory=\"logs\" prefix=\"localhost_access_log\" suffix=\".txt\" pattern=\"%h %l %u %t &quot;%r&quot; %s %b\" />\n"
                                     "      </Host>\n"
                                     "    </Engine>\n"
                                     "  </Service>\n"
                                     "</Server>\n";

const char *TOMCAT_APP_TEMPLATE = R"(<Context path="/${context-path}" docBase="${app-path}" />)";

#if defined(WIN64) || defined(WIN32)
const char *TOMCAT_ENV_CMD = "set JAVA_HOME=${java-home}&set JRE_HOME=&set CATALINA_HOME=${catalina-home}&set CATALINA_BASE=${catalina-base}&set JAVA_OPTS=${jvm-opts}";
#else
const char *TOMCAT_ENV_CMD = "JAVA_HOME=${java-home}&JRE_HOME=&CATALINA_HOME=${catalina-home}&CATALINA_BASE=${catalina-base}&JAVA_OPTS=${jvm-opts}";
#endif

#if defined(WIN32) || defined(WIN64)
const char *TOMCAT_CATALINA_CMD = "\\bin\\catalina.bat";
#else
const char *TOMCAT_CATALINA_CMD = "/bin/catalina.sh";
#endif
