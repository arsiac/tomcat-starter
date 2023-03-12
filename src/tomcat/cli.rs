use log::log_enabled;

use crate::tomcat::RuntimeVersion;
use crate::util::os_utils;
use log::{trace, Level};
use std::path::Path;
use std::process::Command;

pub struct CommandLineBuilder {
    version: RuntimeVersion,
    separate: bool,
    commands: Vec<String>,
}

impl CommandLineBuilder {
    pub fn new(version: &RuntimeVersion, separate: bool) -> Self {
        Self {
            version: version.clone(),
            separate,
            commands: Vec::new(),
        }
    }

    pub fn with_java_home(&mut self, java_home: &Path) -> &mut Self {
        let java_exe = os_utils::get_java(java_home);
        let cmd = java_exe.to_str().unwrap();
        self.commands.push(cmd.to_string());
        self.commands.push("-server".to_string());
        self
    }

    pub fn with_java_opts(&mut self, java_opts: &str) -> &mut Self {
        for opt in java_opts.split(" -") {
            let opt = opt.trim();
            if opt.starts_with("-") {
                self.commands.push(opt.to_string());
            } else {
                self.commands.push(format!("-{}", opt));
            }
        }
        self
    }

    pub fn with_debug(&mut self, port: i32) -> &mut Self {
        let cmd = format!(
            "-agentlib:jdwp=transport=dt_socket,address=0.0.0.0:{},server=y,suspend=n",
            port
        );
        self.commands.push(cmd);
        self
    }

    pub fn with_log(&mut self, logfile: Option<&Path>) -> &mut Self {
        if let Some(file) = logfile {
            if file.exists() {
                let file = file.to_str().unwrap();
                self.commands
                    .push(format!("-Djava.util.logging.config.file={}", file))
            }
        }
        // 会使用系统默认的编码输出日志
        // 但不会记录到日志文件
        self.commands
            .push("-Djava.util.logging.manager=org.apache.juli.ClassLoaderLogManager".to_string());
        self
    }

    pub fn with_catalina(&mut self, home: &Path, base: &Path) -> &mut Self {
        let home_bin = home.join("bin");
        let base_temp = base.join("temp");
        self.commands.push("-Dnop".to_string());
        self.commands
            .push("-Djdk.tls.ephemeralDHKeySize=2048".to_string());
        if self.version.tomcat.major > 7 {
            self.commands
                .push("-Djava.protocol.handler.pkgs=org.apache.catalina.webresources".to_string());
        }
        self.commands.push("-Dignore.endorsed.dirs=".to_string());
        self.commands
            .push(format!("-Dcatalina.home={}", home.to_str().unwrap()).to_string());
        self.commands
            .push(format!("-Dcatalina.base={}", base.to_str().unwrap()));
        self.commands
            .push(format!("-Djava.io.tmpdir={}", base_temp.to_str().unwrap()));
        let bootstrap_jar = home_bin.join("bootstrap.jar");
        let tomcat_juli_jar = home_bin.join("tomcat-juli.jar");
        self.commands.push("-classpath".to_string());
        if os_utils::is_windows() {
            self.commands.push(format!(
                "{};{}",
                bootstrap_jar.to_str().unwrap(),
                tomcat_juli_jar.to_str().unwrap()
            ))
        } else {
            self.commands.push(format!(
                "{}:{}",
                bootstrap_jar.to_str().unwrap(),
                tomcat_juli_jar.to_str().unwrap()
            ))
        }
        self
    }

    pub fn build(&mut self, title: &str) -> Command {
        self.commands
            .push("org.apache.catalina.startup.Bootstrap".to_string());
        self.commands.push("start".to_string());
        let mut command = if self.separate {
            if os_utils::is_windows() {
                let mut cmd = Command::new("cmd.exe");
                cmd.arg("/c").arg("START").arg("/I").arg(title.to_string());
                cmd
            } else {
                Command::new("nohup")
            }
        } else {
            Command::new(self.commands[0].as_str())
        };
        if self.version.jvm.major == 1 && self.version.jvm.minor > 8 {
            let options = "--add-opens=java.base/java.lang=ALL-UNNAMED
            --add-opens=java.base/java.io=ALL-UNNAMED
            --add-opens=java.base/java.util=ALL-UNNAMED
            --add-opens=java.base/java.util.concurrent=ALL-UNNAMED
            --add-opens=java.rmi/sun.rmi.transport=ALL-UNNAMED";
            command.env("JDK_JAVA_OPTIONS", options);
        }

        if log_enabled!(Level::Trace) {
            trace!("Command: {}", self.commands.join(" "));
        }

        if self.separate {
            command.args(&self.commands);
        } else {
            command.args(&self.commands[1..]);
        }
        command
    }
}
