pub const JAVA_HOME: &str = "JAVA_HOME";
pub const CATALINA_HOME: &str = "CATALINA_HOME";
pub const CATALINA_BASE: &str = "CATALINA_BASE";
pub const CATALINA_OPTS: &str = "CATALINA_OPTS";
pub const JPDA_ADDRESS: &str = "JPDA_ADDRESS";
pub const TITLE: &str = "TITLE";

#[cfg(target_os = "windows")]
pub const CATALINA_BIN: &str = "bin/catalina.bat";
#[cfg(target_os = "windows")]
pub const JAVA_BIN: &str = "bin/java.exe";

#[cfg(not(target_os = "windows"))]
pub const CATALINA_BIN: &str = "bin/catalina.sh";
#[cfg(not(target_os = "windows"))]
pub const JAVA_BIN: &str = "bin/java";

pub const SERVER_XML: &str = "server.xml";
pub const LOG_CONFIG_FILE: &str = "logging.properties";
