use crate::config::RuntimeConfig;
use crate::app::util::check_port;
use crate::app::AppError;
use crate::app;
use std::path::PathBuf;

pub struct RuntimeConfigCombine<'a> {
    pub configs: Vec<&'a RuntimeConfig>,
}

impl<'a> RuntimeConfigCombine<'a> {
    pub fn new(configs: Vec<&'a RuntimeConfig>) -> RuntimeConfigCombine<'a> {
        Self { configs }
    }

    pub fn java_home(&self) -> Result<&String, AppError> {
        let mut value = None;
        for config in &self.configs {
            if config.java.is_none() {
                continue;
            }
            let java = config.java.as_ref().unwrap();
            if java.java_home.is_some() {
                value = java.java_home.as_ref();
                break;
            }
        }
        match value {
            None => Err(AppError::Action("java_home is not specified".to_string())),
            Some(value) => {
                let java_home: PathBuf = value.into();
                if !java_home.exists() {
                    return Err(AppError::Action(format!(
                        "java_home '{}' does not exist",
                        java_home.display()
                    )));
                }
                let java_exe = java_home.join(app::constant::JAVA_BIN);
                if !java_exe.exists() {
                    return Err(AppError::Action(format!(
                        "java_home '{}' does not contain java executable",
                        java_home.display()
                    )));
                }
                Ok(value)
            }
        }
    }

    pub fn java_options(&self) -> &'a str {
        let mut value = None;
        for config in &self.configs {
            if config.java.is_none() {
                continue;
            }
            let java = config.java.as_ref().unwrap();
            if java.java_options.is_some() {
                value = java.java_options.as_ref();
                break;
            }
        }
        match value {
            None => "",
            Some(value) => value,
        }
    }

    pub fn tomcat_home(&self) -> Result<&String, AppError> {
        let mut value = None;
        for config in &self.configs {
            if config.tomcat.is_none() {
                continue;
            }
            let tomcat = config.tomcat.as_ref().unwrap();
            if tomcat.tomcat_home.is_some() {
                value = tomcat.tomcat_home.as_ref();
                break;
            }
        }
        match value {
            None => Err(AppError::Action("tomcat_home is not specified".to_string())),
            Some(value) => {
                let tomcat_home: PathBuf = value.into();
                if !tomcat_home.exists() {
                    return Err(AppError::Action(format!(
                        "tomcat_home '{}' does not exist",
                        tomcat_home.display()
                    )));
                }
                let catalina_bin = tomcat_home.join(app::constant::CATALINA_BIN);
                if !catalina_bin.exists() {
                    return Err(AppError::Action(format!(
                        "tomcat_home '{}' does not contain catalina executable",
                        tomcat_home.display()
                    )));
                }
                Ok(value)
            }
        }
    }

    pub fn http_port(&self) -> Result<u32, AppError> {
        let mut value = None;
        for config in &self.configs {
            if config.tomcat.is_none() {
                continue;
            }
            let tomcat = config.tomcat.as_ref().unwrap();
            if tomcat.http_port.is_some() {
                value.clone_from(&tomcat.http_port);
                break;
            }
        }
        match value {
            None => Err(AppError::Action("http_port is not specified".to_string())),
            Some(value) => check_port("http_port", value),
        }
    }

    pub fn server_port(&self) -> Result<u32, AppError> {
        let mut value = None;
        for config in &self.configs {
            if config.tomcat.is_none() {
                continue;
            }
            let tomcat = config.tomcat.as_ref().unwrap();
            if tomcat.server_port.is_some() {
                value.clone_from(&tomcat.server_port);
                break;
            }
        }

        match value {
            None => Err(AppError::Action("server_port is not specified".to_string())),
            Some(value) => check_port("server_port", value),
        }
    }

    pub fn jpda_port(&self) -> Result<u32, AppError> {
        let mut value = None;
        for config in &self.configs {
            if config.tomcat.is_none() {
                continue;
            }
            let tomcat = config.tomcat.as_ref().unwrap();
            if tomcat.jpda_port.is_some() {
                value.clone_from(&tomcat.jpda_port);
                break;
            }
        }

        match value {
            None => Err(AppError::Action("jpda_port is not specified".to_string())),
            Some(value) => check_port("jpda_port", value),
        }
    }
}
