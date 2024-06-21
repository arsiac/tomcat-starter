use crate::action::run::combine::RuntimeConfigCombine;
use std::path::{Path, PathBuf};
use std::process::Stdio;

use crate::app;
use crate::config;
use crate::config::{ProjectItemConfig, TmsConfig};
use crate::app::util::check_port;
use crate::app::AppError;
use crate::app::arg::ActionRun;

mod combine;

pub fn run_project(action: &ActionRun, config: &TmsConfig) -> Result<(), AppError> {
    let project = super::get_project(config, action.project.as_str())?;
    let items = if action.all_items {
        project.items.clone()
    } else {
        let mut items = Vec::new();
        for item_name in &action.items {
            let item = super::get_project_item(project, item_name)?;
            items.push(item.clone());
        }
        items
    };

    for item in &items {
        if !PathBuf::from(&item.path).exists() {
            return Err(AppError::Action(format!(
                "item '{}' does not exist",
                item.name
            )));
        }
    }

    let mut runtimes = Vec::new();
    if project.runtime.is_some() {
        runtimes.push(project.runtime.as_ref().unwrap());
    }

    runtimes.push(&config.default);
    let runtime = RuntimeConfigCombine::new(runtimes);
    let java_home = runtime.java_home()?;
    let java_options = runtime.java_options();
    let tomcat_home = runtime.tomcat_home()?;
    let http_port = if action.http_port.is_some() {
        check_port("http_port", action.http_port.unwrap())?
    } else {
        runtime.http_port()?
    };
    let server_port = if action.server_port.is_some() {
        check_port("server_port", action.server_port.unwrap())?
    } else {
        runtime.server_port()?
    };
    let jpda_port = if action.jpda_port.is_some() {
        check_port("jpda_port", action.jpda_port.unwrap())?
    } else {
        runtime.jpda_port()?
    };

    let item_names: Vec<_> = items.iter().map(|t| t.name.as_str()).collect();
    log::info!(
        "Start project '{}' with items {:?}",
        &project.name,
        &item_names
    );

    let project_cache = config::get_cache_dir().join(project.name.as_str());
    if !project_cache.exists() {
        log::info!(
            "Create project cache directory: {}",
            project_cache.display()
        );
        app::util::create_dirs(project_cache.as_path())?;
    } else {
        log::info!("Clean project cache: {}", project_cache.display());
        app::util::remove_dir_items(project_cache.as_path())?;
    }

    let tomcat_conf: PathBuf = PathBuf::from(tomcat_home).join("conf");
    let cache_conf = project_cache.clone().join("conf");
    if !cache_conf.exists() {
        app::util::create_dirs(cache_conf.as_path())?;
    }
    let cache_temp = project_cache.clone().join("temp");
    if !cache_temp.exists() {
        app::util::create_dirs(cache_temp.as_path())?;
    }

    copy_tomcat_conf(tomcat_conf.as_path(), cache_conf.as_path())?;
    write_tomcat_server_config(&items, http_port, server_port, cache_conf)?;

    let catalina_exe = PathBuf::from(tomcat_home).join(app::constant::CATALINA_BIN);
    let mut command = std::process::Command::new(catalina_exe.display().to_string());
    command
        .env(app::constant::JAVA_HOME, java_home)
        .env(app::constant::CATALINA_HOME, tomcat_home)
        .env(app::constant::CATALINA_BASE, project_cache.display().to_string())
        .env(app::constant::CATALINA_OPTS, java_options)
        .env(app::constant::TITLE, project.name.as_str());

    if action.debug {
        command
            .env(app::constant::JPDA_ADDRESS, format!("localhost:{}", jpda_port))
            .arg("jpda");
    }

    let child = command
        .arg("run")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|e| AppError::Action(format!("Failed to start tomcat: {}", e)))?;
    if action.debug {
        log::info!(
            "Starting Tomcat > PID: {}, HTTP port: {}, JPDA port: {}",
            child.id(),
            http_port,
            jpda_port
        );
    } else {
        log::info!(
            "Starting Tomcat > PID: {}, HTTP port: {}",
            child.id(),
            http_port
        );
    }
    Ok(())
}

fn write_tomcat_server_config(
    items: &Vec<ProjectItemConfig>,
    http_port: u32,
    server_port: u32,
    cache_conf: PathBuf,
) -> Result<(), AppError> {
    let cache_server_xml = cache_conf.join(app::constant::SERVER_XML);
    let mut server_xml_doc = xml_doc::Document::parse_file(cache_server_xml.as_path())
        .map_err(|e| AppError::Action(format!("Failed to parse server.xml: {:?}", e)))?;
    let root_element = server_xml_doc.root_element().unwrap();
    root_element.set_attribute(&mut server_xml_doc, "port", server_port.to_string());
    for service_element in root_element.find_all(&server_xml_doc, "Service") {
        if service_element.attribute(&server_xml_doc, "name") == Some("Catalina") {
            for connector_element in service_element.find_all(&server_xml_doc, "Connector") {
                if connector_element.attribute(&server_xml_doc, "protocol") == Some("HTTP/1.1") {
                    connector_element.set_attribute(
                        &mut server_xml_doc,
                        "port",
                        http_port.to_string(),
                    );
                }
            }

            for engine_element in service_element.find_all(&server_xml_doc, "Engine") {
                if engine_element.attribute(&server_xml_doc, "name") == Some("Catalina") {
                    if let Some(host_element) = engine_element.find(&server_xml_doc, "Host") {
                        for item in items {
                            let context_element =
                                xml_doc::Element::build(&mut server_xml_doc, "Context")
                                    .attribute("docBase", item.path.to_string())
                                    .attribute("path", item.context_path.as_str())
                                    .attribute("reloadable", "true")
                                    .finish();
                            host_element
                                .push_child(
                                    &mut server_xml_doc,
                                    xml_doc::Node::Element(context_element),
                                )
                                .map_err(|e| {
                                    AppError::Action(format!(
                                        "Failed to add context to server.xml: {:?}",
                                        e
                                    ))
                                })?;
                        }
                    }
                }
            }
        }
    }

    // save server.xml
    let new_server_xml = server_xml_doc
        .write_str()
        .map_err(|e| AppError::Action(format!("Failed to write server.xml: {:?}", e)))?;
    std::fs::write(cache_server_xml.as_path(), new_server_xml)
        .map_err(|e| AppError::Action(format!("Failed to write server.xml: {:?}", e)))?;
    Ok(())
}

fn copy_tomcat_conf(tomcat_conf: &Path, cache_conf: &Path) -> Result<(), AppError> {
    match std::fs::read_dir(tomcat_conf) {
        Ok(dir) => {
            for entry in dir {
                if let Err(e) = entry {
                    return Err(AppError::Action(format!(
                        "Failed to read tomcat configuration files: {}",
                        e
                    )));
                }
                let entry = entry.unwrap();
                let entry_path = entry.path();
                let filename = entry_path.file_name().unwrap();
                if filename == app::constant::LOG_CONFIG_FILE {
                    continue;
                }
                let cache_path = cache_conf.join(filename);
                log::debug!(
                    "Copy tomcat configuration file: {} -> {}",
                    entry_path.display(),
                    cache_path.display()
                );
                if let Err(e) = std::fs::copy(entry_path.as_path(), cache_path) {
                    return Err(AppError::Action(format!(
                        "Failed to copy tomcat configuration file: {}",
                        e
                    )));
                }
            }
        }
        Err(e) => {
            return Err(AppError::Action(format!(
                "Failed to read tomcat conf directory: {}",
                e
            )));
        }
    }
    Ok(())
}
