use crate::config::TmsCommon;
use crate::env_ini::{Ini, IniSection};
use crate::env_ini::{IniLoader, IniPathLoad, IniStrLoad};
use crate::util::os_utils;
use log::{debug, error, log_enabled, trace, warn, Level};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct ProjectItem {
    pub name: String,
    pub alias: HashSet<String>,
    pub context_path: String,
    pub source_path: String,
}

#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
    pub alias: HashSet<String>,
    pub description: Option<String>,
    pub runtime: ProjectRuntime,
    pub items: HashMap<String, ProjectItem>,
}

#[derive(Debug, Clone)]
pub struct ProjectRuntime {
    pub java_home: Option<PathBuf>,
    pub java_opts: Option<String>,
    pub catalina_home: Option<PathBuf>,
    pub catalina_base: Option<PathBuf>,
    pub enable_logfile: Option<bool>,
    pub http_port: Option<i32>,
    pub server_port: Option<i32>,
    pub jpda_port: Option<i32>,
}

impl ProjectItem {
    pub fn new() -> ProjectItem {
        ProjectItem {
            name: String::new(),
            alias: HashSet::new(),
            context_path: String::new(),
            source_path: String::new(),
        }
    }
}

impl Project {
    pub fn new() -> Project {
        Project {
            name: String::new(),
            alias: HashSet::new(),
            description: None,
            runtime: ProjectRuntime::new(),
            items: HashMap::new(),
        }
    }

    pub fn get_item(&self, s: &str) -> Option<&ProjectItem> {
        match self.items.get(s) {
            None => {
                for (_, item) in &self.items {
                    if item.alias.contains(s) {
                        return Some(item);
                    }
                }
                None
            }
            Some(i) => Some(i),
        }
    }
}

impl ProjectRuntime {
    pub fn new() -> Self {
        ProjectRuntime {
            java_home: None,
            java_opts: None,
            catalina_home: None,
            catalina_base: None,
            enable_logfile: None,
            http_port: None,
            server_port: None,
            jpda_port: None,
        }
    }
}

/// 校验并获取 java_home 配置
fn get_java_home(java_home: Option<&String>) -> Option<PathBuf> {
    if let Some(java_home) = java_home {
        let java_home_path = PathBuf::from(java_home);
        let execute = os_utils::get_java(java_home_path.as_path());
        if !execute.exists() {
            error!("Invalid JAVA_HOME: {}", java_home);
            exit(1);
        }
        return Some(java_home_path);
    }
    None
}

/// 校验并获取 catalina_home 配置
fn get_catalina_home(catalina_home: Option<&String>) -> Option<PathBuf> {
    if let Some(catalina_home) = catalina_home {
        let catalina_home_path = PathBuf::from(catalina_home);
        let execute = os_utils::get_catalina(catalina_home_path.as_path());
        if !execute.exists() {
            error!("Invalid CATALINA_HOME: {}", catalina_home);
            exit(1);
        }
        return Some(catalina_home_path);
    }
    None
}

/// 校验并获取端口
fn get_port(port: Option<&String>) -> Option<i32> {
    if let Some(port) = port {
        return match i32::from_str(port.as_str()) {
            Ok(val) => {
                if 0 > val || val > 65535 {
                    error!("Invalid port: {}.", port);
                    return None;
                }
                Some(val)
            }
            Err(_) => {
                error!("Parse port failed: {}.", port);
                None
            }
        };
    }
    None
}

fn ini_load_item(project: &mut Project, section: &IniSection) {
    const ITEM_PREFIX: &str = "item.";
    let mut item_map: HashMap<String, ProjectItem> = HashMap::new();
    let mut alias_set: HashSet<String> = HashSet::new();
    let mut context_set: HashSet<String> = HashSet::new();

    for (key, value) in section.iter() {
        if !key.starts_with(ITEM_PREFIX) {
            continue;
        }
        let key_info: Vec<&str> = key.split(".").collect();
        if key_info.len() != 3 {
            warn!("ignore illegal key '{}'.", key);
            continue;
        }
        let item_name = key_info[1];
        let mut item = if item_map.contains_key(item_name) {
            item_map.get(item_name).unwrap().clone()
        } else {
            ProjectItem::new()
        };
        item.name = String::from(item_name);
        trace!(
            "Load project item '{}' of project '{}'",
            &item.name,
            &project.name
        );
        match key_info[2] {
            "alias" => {
                trace!("Load item '{}' alias.", &item.name);
                if let Some(alias) = value {
                    for alias in alias.split(",") {
                        let alias = alias.trim();
                        if !alias_set.insert(String::from(alias)) {
                            error!(
                                "Project '{}', item '{}': Duplicate alias: {}",
                                &project.name, &item.name, alias
                            );
                        }
                        item.alias.insert(String::from(alias));
                    }
                }
            }
            "context_path" => {
                trace!("Load item '{}' context_path.", &item.name);
                match value {
                    None => {
                        error!("'item.{}.context_path' must not be empty.", &item.name);
                        exit(1);
                    }
                    Some(cp) => {
                        if cp.starts_with("/") {
                            if !context_set.insert(cp.clone()) {
                                error!(
                                    "Project '{}', item '{}': Duplicate context_path: {}",
                                    &project.name, &item.name, cp
                                );
                                exit(1);
                            }
                            item.context_path = cp.clone();
                        } else {
                            error!(
                                "'item.{}.context_path' must start with '/': {}",
                                &item.name, cp
                            );
                            exit(1);
                        }
                    }
                }
            }
            "source_path" => {
                trace!("Load item '{}' source_path.", &item.name);
                match value {
                    None => {
                        error!("'item.{}.source_path' must not be empty.", &item.name);
                        exit(1);
                    }
                    Some(sp) => {
                        if PathBuf::from(sp).exists() {
                            item.source_path = sp.clone();
                        } else {
                            error!("File or directory not exists({}): {}", &item.name, sp);
                            exit(1);
                        }
                    }
                }
            }
            _ => {}
        }
        item_map.insert(item.name.clone(), item);
    }

    for (_, item) in item_map.iter() {
        if item.context_path.is_empty() {
            error!("'item.{}.context_path' must not be empty.", &item.name);
            exit(1);
        }

        if item.source_path.is_empty() {
            error!("'item.{}.source_path' must not be empty.", &item.name);
            exit(1);
        }

        project.items.insert(item.name.clone(), item.clone());
    }
}

fn ini_load(ini: &Ini) -> HashMap<String, Project> {
    const PROJECT_PREFIX: &str = "project ";
    let mut projects = HashMap::new();
    let mut alias_set: HashSet<String> = HashSet::new();
    for (name, section) in ini.iter() {
        if name.starts_with(PROJECT_PREFIX) {
            let mut project = Project::new();
            let mut project_name = &name.as_str()[PROJECT_PREFIX.len()..name.len()];
            if project_name.starts_with("\"") {
                project_name = &project_name[1..project_name.len()];
            }
            if project_name.ends_with("\"") {
                project_name = &project_name[0..project_name.len() - 1];
            }
            project.name = String::from(project_name);
            trace!("Load project: {}", &project.name);
            project.description = match section.get("description") {
                None => None,
                Some(desc) => Some(desc.clone()),
            };

            if let Some(val) = section.get("alias") {
                trace!("Load project.alias: {}", val);
                for alias in val.split(",") {
                    let alias = alias.trim();
                    if !alias_set.insert(String::from(alias)) {
                        error!("Project '{}': Duplicate alias: {}", &project.name, alias);
                        exit(1);
                    }
                    project.alias.insert(String::from(alias));
                }
            }

            project.runtime.java_home = get_java_home(section.get("java_home"));
            project.runtime.java_opts = match section.get("java_opts") {
                None => None,
                Some(val) => Some(val.clone()),
            };
            project.runtime.catalina_home = get_catalina_home(section.get("catalina_home"));
            project.runtime.http_port = get_port(section.get("http_port"));
            project.runtime.server_port = get_port(section.get("server_port"));
            project.runtime.jpda_port = get_port(section.get("jpda_port"));

            if let Some(val) = section.get("enable_logfile") {
                project.runtime.enable_logfile = Some("true".eq_ignore_ascii_case(val));
            }

            ini_load_item(&mut project, section);
            if let Some(ref old) = projects.insert(project.name.clone(), project) {
                error!("Duplicate project '{}'", old.name);
                exit(0);
            }
        }
    }

    projects
}

pub fn ini_load_all(config: &TmsCommon, ini: &Ini) -> HashMap<String, Project> {
    let mut projects = HashMap::new();
    for (name, project) in ini_load(ini) {
        if projects.insert(name.clone(), project).is_some() {
            error!("Duplicate project: {}", &name);
            exit(1);
        }
    }

    if let Some(section) = ini.get("project") {
        match section.get("include") {
            None => {
                debug!("No ini include");
            }
            Some(include) => {
                debug!("Load ini: {}", include);
                for file in include.split(",") {
                    let file = if os_utils::is_windows() {
                        file.replace("/", "\\")
                    } else {
                        file.replace("\\", "/")
                    };
                    let path = PathBuf::from(file.as_str());
                    let mut loader = IniLoader::new(false);
                    let include_ini = if path.is_absolute() {
                        if log_enabled!(Level::Trace) {
                            trace!("Path is absolute: {}", path.to_str().unwrap());
                        }
                        IniPathLoad::load(&mut loader, path.as_path())
                    } else {
                        if log_enabled!(Level::Trace) {
                            trace!("Path is relative: {}", path.to_str().unwrap());
                        }
                        let cache_path = PathBuf::from(config.cache_dir.clone());
                        let path = match cache_path.parent() {
                            None => cache_path.as_path(),
                            Some(cp) => cp,
                        }
                        .to_path_buf()
                        .join(file.as_str());
                        if log_enabled!(Level::Trace) {
                            trace!("File path: {}", path.to_str().unwrap());
                        }
                        IniStrLoad::load(&mut loader, path.to_str().unwrap())
                    };
                    match include_ini {
                        Ok(include_ini) => {
                            for (name, project) in ini_load(include_ini) {
                                if projects.insert(name.clone(), project).is_some() {
                                    error!("Duplicate project: {}", &name);
                                    exit(1);
                                }
                            }
                        }
                        Err(ref e) => {
                            error!("Load '{}' failed: {}", &file, e);
                            exit(1);
                        }
                    }
                }
            }
        }
    }
    projects
}
