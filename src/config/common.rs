use crate::env_ini::Ini;
use crate::util::os_utils;
use log::LevelFilter::Info;
use log::{error, log_enabled, warn, Level, LevelFilter, trace};
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;

#[derive(Debug)]
pub struct TmsCommon {
    pub log_level: LevelFilter,
    pub cache_dir: PathBuf,
}

#[derive(Debug)]
pub struct TmsRuntime {
    pub java_home: PathBuf,
    pub java_opts: Option<String>,
    pub catalina_home: PathBuf,
    pub enable_logfile: bool,
    pub http_port: i32,
    pub server_port: i32,
    pub jpda_port: i32,
}

impl TmsCommon {
    pub fn new() -> TmsCommon {
        TmsCommon {
            log_level: Info,
            cache_dir: PathBuf::new(),
        }
    }
}

/// 默认缓存文件夹
fn default_cache_dir() -> PathBuf {
    let cache_dir = os_utils::get_program_folder().join("cache");

    if !cache_dir.exists() {
        if let Err(e) = std::fs::create_dir(cache_dir.as_path()) {
            if log_enabled!(Level::Error) {
                error!("Create default cache directory failed: {}", e.to_string());
            }
            exit(1);
        }
    }

    if let Some(val) = cache_dir.to_str() {
        const TEMP: &str = "\\\\?\\";
        if val.starts_with(TEMP) {
            PathBuf::from(&val[TEMP.len()..val.len()])
        } else {
            cache_dir
        }
    } else {
        error!("Default cache directory is empty.");
        exit(1);
    }
}

impl From<&Ini> for TmsCommon {
    /// 从 env_ini 读取出的配置中初始化
    fn from(ini: &Ini) -> Self {
        let mut tc = TmsCommon::new();
        match ini.get("common") {
            None => {
                env_logger::builder()
                    .filter_level(tc.log_level)
                    .format_timestamp(None)
                    .init();
                return tc;
            }
            Some(section) => {
                if let Some(log_level) = section.get("log_level") {
                    tc.log_level = match LevelFilter::from_str(log_level.as_str()) {
                        Ok(level) => level,
                        Err(_) => {
                            warn!(
                                "Log level '{}' not support. Use default log level 'Info'.",
                                log_level
                            );
                            Info
                        }
                    }
                }

                // 初始化日志
                env_logger::builder()
                    .filter_level(tc.log_level)
                    .format_timestamp(None)
                    .init();

                // cache directory
                tc.cache_dir = match section.get("cache_dir") {
                    None => default_cache_dir(),
                    Some(cache_dir) => {
                        let cache_path = PathBuf::from(cache_dir);
                        if cache_path.exists() {
                            cache_path
                        } else if let Err(e) = std::fs::create_dir(cache_path.as_path()) {
                            if log_enabled!(Level::Warn) {
                                warn!(
                                    "Create cache directory failed: {}: {}",
                                    cache_dir,
                                    e.to_string()
                                );
                            }
                            default_cache_dir()
                        } else {
                            cache_path
                        }
                    }
                };

                tc
            }
        }
    }
}

impl TmsRuntime {
    /// 创建空实例
    pub fn new() -> TmsRuntime {
        TmsRuntime {
            java_home: PathBuf::new(),
            java_opts: None,
            catalina_home: PathBuf::new(),
            enable_logfile: false,
            http_port: 8080,
            server_port: 8005,
            jpda_port: 5005,
        }
    }
}

/// 校验并获取 java_home 配置
fn get_java_home(java_home: Option<&String>) -> PathBuf {
    match java_home {
        None => {
            error!("Please configure JAVA_HOME with 'runtime.java_home' in the configuration file");
            exit(1);
        }
        Some(java_home) => {
            let java_home_path = PathBuf::from(java_home);
            let execute = os_utils::get_java(java_home_path.as_path());
            if !execute.exists() {
                error!("Invalid JAVA_HOME: {}", java_home);
                exit(1);
            }
            java_home_path
        }
    }
}

/// 校验并获取 catalina_home 配置
fn get_catalina_home(catalina_home: Option<&String>) -> PathBuf {
    match catalina_home {
        None => {
            error!(
                "Please configure JAVA_HOME with 'runtime.catalina_home' in the configuration file"
            );
            exit(1);
        }
        Some(catalina_home) => {
            let catalina_home_path = PathBuf::from(catalina_home);
            let execute = os_utils::get_catalina(catalina_home_path.as_path());
            if !execute.exists() {
                error!("Invalid CATALINA_HOME: {}", catalina_home);
                exit(1);
            }
            catalina_home_path
        }
    }
}

/// 校验并获取端口
fn get_port(port: Option<&String>, default_port: i32) -> i32 {
    match port {
        None => default_port,
        Some(port) => match i32::from_str(port.as_str()) {
            Ok(val) => {
                if 0 > val || val > 65535 {
                    error!("Invalid port: {}. Use default port {}.", port, default_port);
                    return default_port;
                }
                val
            }
            Err(_) => {
                error!(
                    "Parse port failed: {}. Use default port {}.",
                    port, default_port
                );
                default_port
            }
        },
    }
}

impl From<&Ini> for TmsRuntime {
    /// 从 env_ini 读取出的配置中初始化
    fn from(ini: &Ini) -> Self {
        let mut tr = TmsRuntime::new();
        match ini.get("runtime") {
            None => tr,
            Some(section) => {
                tr.java_home = get_java_home(section.get("java_home"));
                tr.catalina_home = get_catalina_home(section.get("catalina_home"));
                tr.http_port = get_port(section.get("http_port"), tr.http_port);
                tr.server_port = get_port(section.get("server_port"), tr.server_port);
                tr.jpda_port = get_port(section.get("jpda_port"), tr.jpda_port);

                if let Some(val) = section.get("java_opts") {
                    trace!("[Init TmsRuntime] java_opts: {}", val);
                    tr.java_opts = Some(val.clone());
                }

                if let Some(val) = section.get("enable_logfile") {
                    tr.enable_logfile = "true".eq_ignore_ascii_case(val);
                }

                tr
            }
        }
    }
}
