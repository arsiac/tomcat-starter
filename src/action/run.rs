use crate::action::Actions;
use crate::argument::TmsArgActionRun;
use crate::config::configuration::TmsConfiguration;
use crate::config::project::{Project, ProjectItem};
use crate::tomcat::CommandLineBuilder;
use crate::tomcat::ServerXml;
use crate::tomcat::Tomcat;
use log::{debug, error, info, log_enabled, warn, Level};
use std::io;
use std::io::BufRead;
use std::process::{exit, Stdio};

pub struct RunAction {
    argument: Box<TmsArgActionRun>,
    configuration: Box<TmsConfiguration>,
}

pub struct Items {
    pub items: Vec<ProjectItem>,
    pub names: Vec<String>,
}

impl Items {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            names: Vec::new(),
        }
    }
}

impl RunAction {
    pub fn new(argument: Box<TmsArgActionRun>, configuration: Box<TmsConfiguration>) -> Self {
        Self {
            argument,
            configuration,
        }
    }

    /// 获取要运行的项目
    pub fn get_project(&self) -> Option<&Project> {
        self.configuration.get_project(&self.argument.project)
    }

    /// 获取要运行的子项
    pub fn get_items(&self, project: &Project) -> Option<Items> {
        let mut items = Items::new();
        if self.argument.all_item {
            for item in project.items.values() {
                items.items.push(item.clone());
                items.names.push(item.name.clone());
            }
        } else {
            if self.argument.item.is_empty() {
                error!("Missing --all-item(-a) or --item(-i) <ITEM>.");
                return None;
            }
            for info in &self.argument.item {
                match project.get_item(info.as_str()) {
                    None => {
                        error!("Item not available: {}", info);
                        exit(1);
                    }
                    Some(item) => {
                        items.items.push(item.clone());
                        items.names.push(item.name.clone());
                    }
                }
            }
        }
        Some(items)
    }

    /// 合并运行环境配置
    fn merge_runtime(&self, project: &mut Project) {
        let pr = &mut project.runtime;
        let tr = &self.configuration.runtime;
        let tc = &self.configuration.common;
        if pr.java_home.is_none() {
            pr.java_home = Some(tr.java_home.clone());
        }
        match &pr.java_opts {
            None => {
                if tr.java_opts.is_some() {
                    pr.java_opts = tr.java_opts.clone();
                }
            }
            Some(p_val) => {
                if let Some(t_val) = &tr.java_opts {
                    pr.java_opts = Some(format!("{} {}", t_val, p_val));
                }
            }
        }
        if pr.catalina_home.is_none() {
            pr.catalina_home = Some(tr.catalina_home.clone());
        }
        pr.catalina_base = Some(tc.cache_dir.join(project.name.as_str()));
        if pr.http_port.is_none() {
            pr.http_port = Some(tr.http_port);
        }
        if pr.server_port.is_none() {
            pr.server_port = Some(tr.server_port);
        }
        if pr.jpda_port.is_none() {
            pr.jpda_port = Some(tr.jpda_port);
        }
        if pr.enable_logfile.is_none() {
            pr.enable_logfile = Some(tr.enable_logfile);
        }
    }

    /// 输出程序运行环境
    fn output_runtime(&self, project: &Project, items: &Items) {
        if log_enabled!(Level::Info) {
            let runtime = &project.runtime;
            info!(
                "Run project '{}' with item {:?}",
                &project.name, &items.names
            );
            info!(
                "JDK/JRE            : {}",
                runtime.java_home.as_ref().unwrap().to_str().unwrap()
            );
            info!(
                "JAVA_OPTS          : {}",
                &runtime.java_opts.as_ref().unwrap()
            );
            info!(
                "Tomcat             : {}",
                runtime.catalina_home.as_ref().unwrap().to_str().unwrap()
            );
            info!("Tomcat Http Port   : {}", runtime.http_port.unwrap());
            info!("Tomcat Server Port : {}", runtime.server_port.unwrap());
            if self.argument.debug {
                info!("Tomcat JPDA Port   : {}", runtime.jpda_port.unwrap());
            }
            if !runtime.enable_logfile.unwrap() {
                info!("Tomcat logs will not be saved to the log file.");
            }
        }
    }
}

impl Actions for RunAction {
    fn process(&self) -> bool {
        // 获取项目
        let project = self.get_project();
        if project.is_none() {
            error!("Project '{}' not exists.", &self.argument.project);
            return false;
        }
        let mut project = project.unwrap().clone();

        // 获取子项
        let items = self.get_items(&project);
        if items.is_none() {
            return false;
        }
        let items = items.unwrap();

        // 合并环境
        self.merge_runtime(&mut project);
        let runtime = &project.runtime;

        // 输出环境信息
        self.output_runtime(&project, &items);
        let java_home = runtime.java_home.as_ref().unwrap();
        let cat_home = runtime.catalina_home.as_ref().unwrap();
        let cat_base = runtime.catalina_base.as_ref().unwrap();
        let tomcat = Tomcat::new(java_home.as_path(), cat_home.as_path(), cat_base.as_path());
        tomcat.init_catalina_base();

        // 获取环境版本
        let runtime_version;
        match tomcat.get_version() {
            Ok(ver) => {
                runtime_version = ver;
            }
            Err(e) => {
                error!("Get runtime version: {}", e.to_string());
                return false;
            }
        }

        // 创建 CATALINA_BASE 的 server.xml
        let mut server_xml = if runtime_version.tomcat.major < 8 {
            ServerXml::new_tomcat7()
        } else {
            ServerXml::new_tomcat8()
        };
        server_xml.http_port(runtime.http_port.unwrap());
        server_xml.server_port(runtime.server_port.unwrap());
        for item in items.items {
            server_xml.add_context(item.context_path.as_str(), item.source_path.as_str())
        }
        tomcat.create_base_server_xml(&server_xml);

        let mut builder = CommandLineBuilder::new(&runtime_version, self.argument.separate);

        builder
            .with_java_home(java_home.as_path())
            .with_catalina(cat_home.as_path(), cat_base.as_path());
        if runtime.enable_logfile.unwrap() {
            let logfile = cat_base.join("conf").join("logging.properties");
            builder.with_log(Some(logfile.as_path()));
        } else {
            builder.with_log(None);
        }
        if let Some(ref java_opts) = runtime.java_opts {
            builder.with_java_opts(java_opts.as_str());
        }
        if self.argument.debug {
            builder.with_debug(runtime.jpda_port.unwrap());
        }

        let title;
        if let Some(ref val) = project.description {
            title = format!("{}: {}", val, items.names.join(","));
        } else {
            title = format!("{}: {}", &project.name, items.names.join(","));
        }

        match builder.build(title.as_str()).stdout(Stdio::piped()).spawn() {
            Ok(mut child) => {
                if self.argument.separate {
                    info!("Tomcat starting ...");
                    return true;
                } else {
                    let stdout = child.stdout.take().unwrap();
                    let mut reader = io::BufReader::new(stdout);
                    loop {
                        let mut out_line = String::new();
                        if let Ok(_) = reader.read_line(&mut out_line) {
                            if let Ok(Some(_)) = child.try_wait() {
                                break;
                            }

                            if out_line.contains("DEBUG") {
                                debug!("{}", &out_line[..out_line.len() - 1]);
                            } else if out_line.contains("INFO") {
                                info!("{}", &out_line[..out_line.len() - 1]);
                            } else if out_line.contains("WARN") {
                                warn!("{}", &out_line[..out_line.len() - 1]);
                            } else if out_line.contains("ERROR") {
                                error!("{}", &out_line[..out_line.len() - 1]);
                            } else {
                                print!("{}", &out_line);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                error!("Start tomcat failed: {}", e.to_string());
                return false;
            }
        }
        true
    }
}
