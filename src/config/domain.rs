#[derive(Debug, Clone)]
pub struct TmsConfig {
    pub default: RuntimeConfig,
    pub projects: Vec<ProjectConfig>,
}

#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub java: Option<JavaConfig>,
    pub tomcat: Option<TomcatConfig>,
}

#[derive(Debug, Clone)]
pub struct JavaConfig {
    pub java_home: Option<String>,
    pub java_options: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TomcatConfig {
    pub tomcat_home: Option<String>,
    pub http_port: Option<u32>,
    pub server_port: Option<u32>,
    pub jpda_port: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct ProjectConfig {
    pub name: String,
    pub alias: Option<String>,
    pub description: Option<String>,
    pub runtime: Option<RuntimeConfig>,
    pub items: Vec<ProjectItemConfig>,
}

#[derive(Debug, Clone)]
pub struct ProjectItemConfig {
    pub name: String,
    pub alias: Option<String>,
    pub path: String,
    pub context_path: String,
}
