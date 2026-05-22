use std::path::PathBuf;

use serde::Deserialize;

use crate::app::AppError;

const CONFIG_FILENAME: &str = "config.toml";

#[derive(Debug, Deserialize)]
pub struct TmsFileConfig {
    pub log_level: Option<String>,
    pub include: Option<Vec<String>>,
    pub default: Option<RuntimeFileConfig>,
    #[serde(rename = "project")]
    pub projects: Option<Vec<ProjectFileConfig>>,
}

#[derive(Debug, Deserialize)]
pub struct RuntimeFileConfig {
    pub java: Option<JavaFileConfig>,
    pub tomcat: Option<TomcatFileConfig>,
}

#[derive(Debug, Deserialize)]
pub struct JavaFileConfig {
    pub java_home: Option<String>,
    pub java_options: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TomcatFileConfig {
    pub tomcat_home: Option<String>,
    pub http_port: Option<u32>,
    pub server_port: Option<u32>,
    pub jpda_port: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct ProjectFileConfig {
    pub name: Option<String>,
    pub alias: Option<String>,
    pub description: Option<String>,
    pub runtime: Option<RuntimeFileConfig>,

    #[serde(rename = "item")]
    pub items: Option<Vec<ProjectItemFileConfig>>,
}

#[derive(Debug, Deserialize)]
pub struct ProjectItemFileConfig {
    pub name: Option<String>,
    pub alias: Option<String>,
    pub path: Option<String>,
    #[serde(rename = "context-path")]
    pub context_path: Option<String>,
}

pub fn load_config_file(config_path: Option<PathBuf>) -> Result<TmsFileConfig, AppError> {
    let (base_dir, config_file) = match config_path {
        Some(path) => {
            let dir = path.parent().map(|p| p.to_path_buf()).unwrap_or_else(super::get_config_dir);
            (dir, path)
        }
        None => {
            let dir = super::get_config_dir();
            (dir.clone(), dir.join(CONFIG_FILENAME))
        }
    };

    if !config_file.exists() {
        return Err(AppError::Config(format!(
            "config file not found: {}",
            config_file.display()
        )));
    }
    let file_content = std::fs::read_to_string(config_file.as_path())
        .map_err(|e| AppError::Config(format!("Failed to read config file: {}", e)))?;

    let mut config = toml::from_str::<TmsFileConfig>(file_content.as_str())
        .map_err(|e| AppError::Config(format!("Failed to parse config file: {}", e)))?;

    if config.projects.is_none() {
        config.projects = Some(Vec::new());
    }
    if let Some(includes) = &config.include {
        let projects = config.projects.as_mut().unwrap();
        for include in includes {
            let include_file = base_dir.join(include);
            if !include_file.exists() {
                return Err(AppError::Config(format!(
                    "include file not found: {}",
                    include_file.display()
                )));
            }
            let file_content = std::fs::read_to_string(include_file.as_path())
                .map_err(|e| AppError::Config(format!("Failed to read include file: {}", e)))?;

            let include_config = toml::from_str::<TmsFileConfig>(file_content.as_str())
                .map_err(|e| AppError::Config(format!("Failed to parse include file: {}", e)))?;

            if let Some(include_projects) = include_config.projects {
                for project_config in include_projects {
                    projects.push(project_config);
                }
            }
        }
    }

    Ok(config)
}
