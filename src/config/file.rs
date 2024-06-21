use serde::Deserialize;

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

pub fn load_config_file() -> TmsFileConfig {
    let config_dir = super::get_config_dir();
    let config_file = config_dir.clone().join(CONFIG_FILENAME);
    if !config_file.exists() {
        panic!("config file not found: {}", config_file.display())
    }
    let file_content = std::fs::read_to_string(config_file.as_path());
    if let Err(e) = file_content {
        panic!("Failed to read config file: {}", e);
    }

    let file_content = file_content.unwrap();
    let config = toml::from_str::<TmsFileConfig>(file_content.as_str());
    if let Err(e) = config {
        panic!("Failed to parse config file: {}", e);
    }
    let mut config = config.unwrap();
    if config.projects.is_none() {
        config.projects = Some(Vec::new());
    }
    if let Some(includes) = &config.include {
        let projects = config.projects.as_mut().unwrap();
        for include in includes {
            let include_file = config_dir.clone().join(include);
            if !include_file.exists() {
                panic!("include file not found: {}", include_file.display())
            }
            let file_content = std::fs::read_to_string(include_file.as_path());
            if let Err(e) = file_content {
                panic!("Failed to read include file: {}", e);
            }

            let file_content = file_content.unwrap();
            let include_config = toml::from_str::<TmsFileConfig>(file_content.as_str());
            if let Err(e) = include_config {
                panic!("Failed to parse include file: {}", e);
            }

            let include_config = include_config.unwrap();
            if let Some(include_projects) = include_config.projects {
                for project_config in include_projects {
                    projects.push(project_config);
                }
            }
        }
    }

    config
}
