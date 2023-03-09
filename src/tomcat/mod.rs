mod cli;
mod server_xml;

pub use crate::tomcat::cli::CommandLineBuilder;
pub use crate::tomcat::server_xml::ServerXml;
use crate::util::file_utils;
use log::{debug, error, log_enabled, trace, Level};
use std::fs::File;
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::str::FromStr;
use std::{fs, io};

const CONF_LIST: [&str; 9] = [
    "catalina.policy",
    "catalina.properties",
    "context.xml",
    "jaspic-providers.xml",
    "jaspic-providers.xsd",
    "logging.properties",
    "tomcat-users.xml",
    "tomcat-users.xsd",
    "web.xml",
];

const TOMCAT_VER_PREFIX: &str = "Server number:";
const JVM_VER_PREFIX: &str = "JVM Version:";

/// Tomcat 版本号: 8.5.85.0, 7.0.109.0, 9.0.73.0
/// JVM 版本号: 1.8.0, 11.0.18, 17.0.6
#[derive(Debug, Clone)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub revise: u32,
}

#[derive(Debug, Clone)]
pub struct RuntimeVersion {
    pub tomcat: Version,
    pub jvm: Version,
}

#[derive(Debug)]
pub struct Tomcat {
    pub catalina_home: PathBuf,
    pub catalina_base: PathBuf,
    pub java_home: PathBuf,
}

impl Version {
    pub fn new() -> Self {
        Self {
            major: 0,
            minor: 0,
            revise: 0,
        }
    }

    /// 解析版本号
    fn parse_version(ver: &str) -> Result<u32, String> {
        if "0" == ver {
            Ok(0)
        } else {
            match u32::from_str(ver) {
                Ok(ver) => Ok(ver),
                Err(e) => Err(e.to_string()),
            }
        }
    }
}

impl FromStr for Version {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let versions: Vec<&str> = s.split(".").collect();
        let mut ver = Version::new();
        if versions.len() > 2 {
            ver.revise = Self::parse_version(versions[2])?;
        }
        if versions.len() > 1 {
            ver.minor = Self::parse_version(versions[1])?;
        }
        ver.major = Self::parse_version(versions[0])?;
        Ok(ver)
    }
}

impl RuntimeVersion {
    pub fn new() -> Self {
        Self {
            tomcat: Version::new(),
            jvm: Version::new(),
        }
    }
}

impl Tomcat {
    pub fn new(java_home: &Path, catalina_home: &Path, catalina_base: &Path) -> Self {
        Self {
            java_home: java_home.to_path_buf(),
            catalina_home: catalina_home.to_path_buf(),
            catalina_base: catalina_base.to_path_buf(),
        }
    }

    /// 创建 server.xml
    pub fn create_base_server_xml(&self, server_xml: &ServerXml) -> bool {
        let server_xml_path = self.catalina_base.join("conf").join("server.xml");
        match File::create(server_xml_path.as_path()) {
            Ok(mut file) => match file.write(server_xml.to_string().as_bytes()) {
                Ok(size) => {
                    trace!("Create server.xml succeed ({} bytes).", size);
                    true
                }
                Err(e) => {
                    if log_enabled!(Level::Error) {
                        error!("Create server.xml failed: {}", e.to_string());
                    }
                    false
                }
            },
            Err(e) => {
                if log_enabled!(Level::Error) {
                    error!("Create server.xml failed: {}", e.to_string());
                }
                false
            }
        }
    }
    /// 获取 JVM/Tomcat 版本
    pub fn get_version(&self) -> Result<RuntimeVersion, String> {
        if log_enabled!(Level::Trace) {
            trace!("JAVA_HOME: {}", self.java_home.to_str().unwrap());
            trace!("CATALINA_HOME: {}", self.catalina_home.to_str().unwrap());
        }
        let java_exe = crate::util::os_utils::get_java(self.java_home.as_path());
        let catalina_jar = self.catalina_home.join("lib").join("catalina.jar");
        let mut cli = Command::new(java_exe.to_str().unwrap());
        cli.arg("-cp")
            .arg(catalina_jar.to_str().unwrap())
            .arg("org.apache.catalina.util.ServerInfo")
            .stdout(Stdio::piped());

        match cli.spawn() {
            Ok(mut child) => {
                let mut runtime_version = RuntimeVersion::new();
                let stdout = child.stdout.take().unwrap();
                let mut reader = io::BufReader::new(stdout);
                loop {
                    let mut out_line = String::new();
                    if let Ok(_) = reader.read_line(&mut out_line) {
                        if let Ok(Some(_)) = child.try_wait() {
                            break;
                        }
                        if out_line.starts_with(TOMCAT_VER_PREFIX) {
                            if log_enabled!(Level::Trace) {
                                trace!("Tomcat {}", &out_line[0..out_line.len() - 1]);
                            }
                            let version = &out_line[TOMCAT_VER_PREFIX.len()..out_line.len()];
                            runtime_version.tomcat = Version::from_str(version.trim())?;
                        } else if out_line.starts_with(JVM_VER_PREFIX) {
                            if log_enabled!(Level::Trace) {
                                trace!("{}", &out_line[0..out_line.len() - 1]);
                            }
                            let version = &out_line[JVM_VER_PREFIX.len()..out_line.len()];
                            let version = version.trim();
                            if version.starts_with("1.8.0") {
                                runtime_version.jvm.major = 8;
                            } else {
                                runtime_version.jvm = Version::from_str(version)?;
                            }
                        }
                    }
                }
                trace!("Runtime Version: {:?}", runtime_version);
                Ok(runtime_version)
            }
            Err(e) => Err(e.to_string()),
        }
    }

    /// 初始化 CATALINA_BASE
    pub fn init_catalina_base(&self) -> bool {
        if let Some(msg) = self.try_init_base() {
            error!("Create CATALINA_BASE failed: {}", msg);
            return false;
        }

        if let Some(msg) = self.try_init_base_conf() {
            error!("Create CATALINA_BASE/conf failed: {}", msg);
            return false;
        }

        if let Some(msg) = self.try_init_base_logs() {
            error!("Create CATALINA_BASE/logs failed: {}", msg);
            return false;
        }

        if let Some(msg) = self.try_init_base_webapps() {
            error!("Create CATALINA_BASE/webapps failed: {}", msg);
            return false;
        }

        self.copy_configs()
    }

    /// 创建 ${CATALINA_BASE}
    fn try_init_base(&self) -> Option<String> {
        if !self.catalina_base.exists() {
            if log_enabled!(Level::Trace) {
                trace!("Create {}", self.catalina_base.to_str().unwrap());
            }
            if let Err(e) = fs::create_dir(self.catalina_base.as_path()) {
                return Some(e.to_string());
            }
        }
        None
    }

    /// 创建 ${CATALINA_BASE}/conf
    fn try_init_base_conf(&self) -> Option<String> {
        let conf_dir = self.catalina_base.join("conf");
        if !conf_dir.exists() {
            if log_enabled!(Level::Trace) {
                trace!("Create {}", conf_dir.to_str().unwrap());
            }
            if let Err(e) = fs::create_dir(conf_dir.as_path()) {
                return Some(e.to_string());
            }
        }
        None
    }

    /// 创建 ${CATALINA_BASE}/logs
    fn try_init_base_logs(&self) -> Option<String> {
        let logs_dir = self.catalina_base.join("logs");
        if !logs_dir.exists() {
            if log_enabled!(Level::Trace) {
                trace!("Create {}", logs_dir.to_str().unwrap());
            }

            if let Err(e) = fs::create_dir(logs_dir.as_path()) {
                return Some(e.to_string());
            }
        }
        None
    }

    /// 创建 ${CATALINA_BASE}/webapps
    fn try_init_base_webapps(&self) -> Option<String> {
        let webapps_dir = self.catalina_base.join("webapps");
        if !webapps_dir.exists() && !file_utils::create_dir(webapps_dir.as_path()) {
            if log_enabled!(Level::Trace) {
                trace!("Create {}", webapps_dir.to_str().unwrap());
            }
            if let Err(e) = fs::create_dir(webapps_dir.as_path()) {
                return Some(e.to_string());
            }
        }
        None
    }

    /// 复制配置文件(除了server.xml)
    fn copy_configs(&self) -> bool {
        let home_conf_dir = self.catalina_home.join("conf");
        let base_conf_dir = self.catalina_base.join("conf");
        for file in CONF_LIST {
            let target_path = base_conf_dir.join(file);
            if !target_path.exists() {
                let source_path = home_conf_dir.join(file);
                if log_enabled!(Level::Trace) {
                    let source = source_path.to_str().unwrap();
                    let target = target_path.to_str().unwrap();
                    trace!("Copy {} to {}", source, target);
                }
                if let Err(e) = fs::copy(source_path.as_path(), target_path.as_path()) {
                    if log_enabled!(Level::Error) {
                        let source = source_path.to_str().unwrap();
                        let target = target_path.to_str().unwrap();
                        let msg = e.to_string();
                        error!("Copy '{}' to '{}' failed: {}", source, target, msg);
                    }
                    return false;
                }
            } else {
                debug!("File already exists: {}", file);
            }
        }
        true
    }
}
