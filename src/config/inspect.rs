use std::str::FromStr;

use crate::app;
use log::LevelFilter;

use crate::config::domain::{
    JavaConfig, ProjectConfig, ProjectItemConfig, RuntimeConfig, TmsConfig, TomcatConfig,
};
use crate::config::file;
use crate::config::file::{
    JavaFileConfig, ProjectFileConfig, ProjectItemFileConfig, RuntimeFileConfig, TomcatFileConfig,
};
use crate::app::AppError;

pub fn init() -> Result<TmsConfig, AppError> {
    let file_config = file::load_config_file();
    init_logger(file_config.log_level.as_ref())?;
    let config = TmsConfig {
        default: init_runtime(file_config.default.as_ref(), true)?,
        projects: init_projects(file_config.projects.as_ref())?,
    };

    Ok(config)
}

fn init_logger(level_text: Option<&String>) -> Result<(), AppError> {
    match level_text {
        Some(value) => match LevelFilter::from_str(value) {
            Ok(level) => {
                app::logger::init(level);
                Ok(())
            }
            Err(e) => {
                app::logger::init(LevelFilter::Warn);
                Err(AppError::Config(format!(
                    "Invalid log level '{}': {}",
                    value, e
                )))
            }
        },
        None => {
            app::logger::init(LevelFilter::Info);
            Ok(())
        }
    }
}

fn init_runtime(runtime_config: Option<&RuntimeFileConfig>, is_default: bool) -> Result<RuntimeConfig, AppError> {
    let runtime_config = match runtime_config {
        Some(runtime_config) => RuntimeConfig {
            java: Some(init_java_config(runtime_config.java.as_ref(), is_default)?),
            tomcat: Some(init_tomcat_config(runtime_config.tomcat.as_ref(), is_default)?),
        },
        None => RuntimeConfig {
            java: Some(init_java_config(None, is_default)?),
            tomcat: Some(init_tomcat_config(None, is_default)?),
        },
    };
    Ok(runtime_config)
}

fn init_java_config(java_config: Option<&JavaFileConfig>, is_default: bool) -> Result<JavaConfig, AppError> {
    let mut java_home = None;
    let mut java_opts = None;
    if let Some(value) = java_config {
        java_home.clone_from(&value.java_home);
        java_opts.clone_from(&value.java_options);
    }

    if java_home.is_none() && is_default {
        log::trace!("No java_home set, falling back to environment variable JAVA_HOME");
        java_home = match std::env::var("JAVA_HOME") {
            Ok(value) => Some(value),
            Err(e) => {
                log::debug!("Failed to get JAVA_HOME: {}", e);
                None
            }
        }
    }

    let java_config = JavaConfig {
        java_home,
        java_options: java_opts,
    };
    Ok(java_config)
}

fn init_tomcat_config(tomcat_config: Option<&TomcatFileConfig>, is_default: bool) -> Result<TomcatConfig, AppError> {
    let mut tomcat_home = None;
    let mut http_port = None;
    let mut server_port = None;
    let mut jpda_port = None;

    if let Some(value) = tomcat_config {
        tomcat_home.clone_from(&value.tomcat_home);
        http_port = value.http_port;
        server_port = value.server_port;
        jpda_port = value.jpda_port;
    }

    if tomcat_home.is_none() && is_default {
        log::trace!("No tomcat_home set, falling back to environment variable CATALINA_HOME");
        tomcat_home = match std::env::var("CATALINA_HOME") {
            Ok(value) => Some(value),
            Err(e) => {
                log::debug!("Failed to get CATALINA_HOME: {}", e);
                None
            }
        };
    }

    if http_port.is_none() {
        log::trace!("No http_port set, falling back to 8080");
        http_port = Some(8080);
    }
    if server_port.is_none() {
        log::trace!("No server_port set, falling back to 8005");
        server_port = Some(8005);
    }

    if jpda_port.is_none() {
        log::trace!("No jpda_port set, falling back to 8000");
        jpda_port = Some(8000);
    }

    let tomcat_config = TomcatConfig {
        tomcat_home,
        http_port,
        server_port,
        jpda_port,
    };
    Ok(tomcat_config)
}

fn init_projects(
    projects: Option<&Vec<ProjectFileConfig>>,
) -> Result<Vec<ProjectConfig>, AppError> {
    if projects.is_none() {
        return Err(AppError::Config("No project defined".to_string()));
    }

    let mut projects_config = Vec::new();
    for (i, project) in projects.unwrap().iter().enumerate() {
        if project.name.is_none() {
            return Err(AppError::Config(format!(
                "No name defined for project {}",
                i + 1
            )));
        }
        let name = project.name.as_ref().unwrap().clone();
        let runtime = match project.runtime.as_ref() {
            None => None,
            Some(runtime) => Some(init_runtime(Some(runtime), false)?),
        };
        let items = init_project_items(&name, project.items.as_ref())?;
        let project_config = ProjectConfig {
            name,
            alias: project.alias.clone(),
            description: project.description.clone(),
            runtime,
            items,
        };
        projects_config.push(project_config);
    }

    Ok(projects_config)
}

fn init_project_items(
    project_name: &str,
    items: Option<&Vec<ProjectItemFileConfig>>,
) -> Result<Vec<ProjectItemConfig>, AppError> {
    if items.is_none() {
        return Err(AppError::Config(format!(
            "No items defined for project '{}'",
            project_name
        )));
    }

    let mut items_config = Vec::new();
    for (i, item) in items.unwrap().iter().enumerate() {
        if item.name.is_none() {
            return Err(AppError::Config(format!(
                "No name defined for project '{}' item {}",
                project_name,
                i + 1
            )));
        }
        let name = item.name.as_ref().unwrap().clone();
        if item.path.is_none() {
            return Err(AppError::Config(format!(
                "No path defined for project '{}' item '{}'",
                project_name, &name
            )));
        }

        let context_path = match item.context_path.as_ref() {
            None => {
                log::debug!(
                    "No context_path defined for project '{}' item '{}', falling back to '/'",
                    project_name,
                    &name
                );
                "/".to_string()
            }
            Some(context_path) => {
                if context_path.starts_with('/') {
                    context_path.clone()
                } else {
                    log::info!(
                        "context_path for project '{}' item '{}' does not start with '/'",
                        project_name,
                        &name
                    );
                    format!("/{}", context_path)
                }
            }
        };
        let item_config = ProjectItemConfig {
            name,
            alias: item.alias.clone(),
            path: item.path.as_ref().unwrap().clone(),
            context_path,
        };
        items_config.push(item_config);
    }
    Ok(items_config)
}
