#include "tmsdef.h"

const char *CONFIG_DIRECTORY = ".tms";
const char *CACHE_DIRECTORY = "tms_cache";
const char *BIN_DIRECTORY = "bin";
const char *CONF_DIRECTORY = "conf";
const char *WEBAPP_DIRECTORY = "webapps";
const char *CONFIG_FILE_NAME = "config.ini";
const char *TOMCAT_CONTEXT_TEMPLATE = R"(<Context path="${context-path}" docBase="${web-document}" reloadable="true"/>)";
const char *TOMCAT_SERVER_TEMPLATE =
    "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n"
    "<Server port=\"${server-port}\" shutdown=\"SHUTDOWN\">\n"
    "  <Listener className=\"org.apache.catalina.startup.VersionLoggerListener\" />\n"
    "  <Listener className=\"org.apache.catalina.core.AprLifecycleListener\" SSLEngine=\"on\" />\n"
    "  <Listener className=\"org.apache.catalina.core.JreMemoryLeakPreventionListener\" />\n"
    "  <Listener className=\"org.apache.catalina.mbeans.GlobalResourcesLifecycleListener\" />\n"
    "  <Listener className=\"org.apache.catalina.core.ThreadLocalLeakPreventionListener\" />\n"
    "  <GlobalNamingResources>\n"
    "    <Resource name=\"UserDatabase\" auth=\"Container\" type=\"org.apache.catalina.UserDatabase\" "
    "description=\"User database that can be updated and saved\" "
    "factory=\"org.apache.catalina.users.MemoryUserDatabaseFactory\" pathname=\"conf/tomcat-users.xml\" />\n"
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
    "        <Valve className=\"org.apache.catalina.valves.AccessLogValve\" directory=\"logs\" "
    "prefix=\"localhost_access_log\" suffix=\".txt\" pattern=\"%h %l %u %t &quot;%r&quot; %s %b\" />\n"
    "      </Host>\n"
    "    </Engine>\n"
    "  </Service>\n"
    "</Server>\n";