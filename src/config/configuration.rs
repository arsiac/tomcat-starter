use crate::config::project;
use crate::config::Project;
use crate::config::{TmsCommon, TmsRuntime};
use crate::env_ini::{IniLoader, IniPathLoad};
use crate::util::tms_utils;
use log::{log_enabled, trace, Level};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

const CONFIG_TEMPLATE: &str = r#"[common]
log_level = info
; cache_dir = 

[runtime]
java_home = ${JAVA_HOME}
java_opts = -XX:+HeapDumpOnOutOfMemoryError \
            -XX:-OmitStackTraceInFastThrow
catalina_home = ${CATALINA_HOME}
enable_logfile = false
http_port = 8080
server_port = 8005
jpda_port = 5005

[project]
; include = project/app3.env_ini \
;           , xxx/app4.env_ini

[project "example"]
alias = ex
description = Example
java_opts = -Dproject.demo=example
jpda_port = 9888
enable_logfile = true
; item `app1`
item.app1.alias = a1
item.app1.context_path = /app-one
item.app1.source_path = /path/to/app1
; item `app2`
item.app2.alias = a2
item.app2.context_path = /app2
item.app2.source_path = /path/to/app2
"#;

#[derive(Debug)]
pub struct TmsConfiguration {
    pub common: TmsCommon,
    pub runtime: TmsRuntime,
    pub projects: HashMap<String, Project>,
}

impl TmsConfiguration {
    pub fn load() -> Result<Box<TmsConfiguration>, String> {
        let config_file = tms_utils::get_tms_home().join("config.ini");
        if !config_file.exists() {
            let error = match File::create(config_file.as_path()) {
                Err(e) => Some(format!("Create default configuration file failed: {}", e)),
                Ok(mut file) => match file.write(CONFIG_TEMPLATE.as_bytes()) {
                    Err(e) => Some(format!("Write default configuration failed. {}", e)),
                    Ok(_) => None,
                },
            };

            if error.is_some() {
                return Err(error.unwrap());
            }
        }

        let config_file = config_file.as_path();
        if log_enabled!(Level::Trace) {
            trace!(
                "Load ini configuration file: {}",
                config_file.to_str().unwrap()
            );
        }
        match IniLoader::new(true).load(config_file) {
            Err(msg) => Err(msg),
            Ok(ini) => {
                let tc = TmsCommon::from(ini);
                let tr = TmsRuntime::from(ini);
                let projects = project::ini_load_all(&tc, ini);
                Ok(Box::new(TmsConfiguration {
                    common: tc,
                    runtime: tr,
                    projects,
                }))
            }
        }
    }

    pub fn get_project(&self, s: &str) -> Option<&Project> {
        match self.projects.get(s) {
            None => {
                for (_, project) in &self.projects {
                    if project.alias.contains(s) {
                        return Some(project);
                    }
                }

                None
            }
            Some(p) => Some(p),
        }
    }
}
