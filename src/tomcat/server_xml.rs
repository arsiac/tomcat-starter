use log::{error, log_enabled, Level};
use serde::{Deserialize, Serialize};
use std::process::exit;

/// <Context path="/context" docBase="/path" reloadable="true"/>
#[derive(Deserialize, Serialize, Debug)]
pub struct Context {
    #[serde(rename = "@path")]
    path: String,
    #[serde(rename = "@docBase")]
    doc_base: String,
    #[serde(rename = "@reloadable")]
    reloadable: bool,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            path: String::new(),
            doc_base: String::new(),
            reloadable: true,
        }
    }
}

impl Context {
    pub fn new(path: &str, doc: &str) -> Self {
        Context {
            path: path.to_string(),
            doc_base: doc.to_string(),
            reloadable: true,
        }
    }
}

/// <Valve className="org.apache.catalina.valves.AccessLogValve"
///        directory="logs"
///        prefix="localhost_access_log"
///        suffix=".txt"
///        pattern="%h %l %u %t &quot;%r&quot; %s %b"
/// />
#[derive(Deserialize, Serialize, Debug)]
pub struct Value {
    #[serde(rename = "@className")]
    class_path: String,
    #[serde(rename = "@directory")]
    directory: String,
    #[serde(rename = "@prefix")]
    prefix: String,
    #[serde(rename = "@suffix")]
    suffix: String,
    #[serde(rename = "@pattern")]
    pattern: String,
}

impl Default for Value {
    fn default() -> Self {
        Value {
            class_path: "org.apache.catalina.valves.AccessLogValve".to_string(),
            directory: "logs".to_string(),
            prefix: "localhost_access_log".to_string(),
            suffix: ".txt".to_string(),
            pattern: "%h %l %u %t \"%r\" %s %b".to_string(),
        }
    }
}

/// <Host name="localhost"  appBase="webapps" unpackWARs="true" autoDeploy="true">
///     <Valve ... />
///     <Context ... />
/// </Host>
#[derive(Deserialize, Serialize, Debug)]
pub struct Host {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@appBase")]
    app_base: String,
    #[serde(rename = "@unpackWARs")]
    unpack_wars: bool,
    #[serde(rename = "@autoDeploy")]
    auto_deploy: bool,
    #[serde(rename = "Value")]
    value: Value,
    #[serde(rename = "Context")]
    contexts: Vec<Context>,
}

impl Default for Host {
    fn default() -> Self {
        Host {
            name: "localhost".to_string(),
            app_base: "webapps".to_string(),
            unpack_wars: true,
            auto_deploy: true,
            value: Value::default(),
            contexts: Vec::new(),
        }
    }
}

/// <Realm className="org.apache.catalina.realm.LockOutRealm">
///     <Realm className="org.apache.catalina.realm.UserDatabaseRealm" resourceName="UserDatabase"/>
/// </Realm>
#[derive(Deserialize, Serialize, Debug)]
pub struct Realm {
    #[serde(rename = "@className")]
    class_name: String,
    #[serde(rename = "Realm")]
    realm: SubRealm,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SubRealm {
    #[serde(rename = "@className")]
    class_name: String,
    #[serde(rename = "@resourceName")]
    resource_name: String,
}

impl Default for Realm {
    fn default() -> Self {
        Realm {
            class_name: "org.apache.catalina.realm.LockOutRealm".to_string(),
            realm: SubRealm {
                class_name: "org.apache.catalina.realm.UserDatabaseRealm".to_string(),
                resource_name: "UserDatabase".to_string(),
            },
        }
    }
}

/// <Engine name="Catalina" defaultHost="localhost">
//       <Realm ... />
//       <Host ... />
/// </Engine>
#[derive(Deserialize, Serialize, Debug)]
pub struct Engine {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@defaultHost")]
    default_host: String,

    #[serde(rename = "Realm")]
    realm: Realm,
    #[serde(rename = "Host")]
    host: Host,
}

impl Default for Engine {
    fn default() -> Self {
        Engine {
            name: "Catalina".to_string(),
            default_host: "localhost".to_string(),
            realm: Realm::default(),
            host: Host::default(),
        }
    }
}

/// <Connector port="8080" protocol="HTTP/1.1" connectionTimeout="20000" redirectPort="8443" />
#[derive(Deserialize, Serialize, Debug)]
pub struct Connector {
    #[serde(rename = "@port")]
    port: i32,
    #[serde(rename = "@protocol")]
    protocol: String,
    #[serde(rename = "@connectionTimeout")]
    connection_timeout: i32,
    #[serde(rename = "@redirectPort")]
    redirect_port: i32,
}

impl Default for Connector {
    fn default() -> Self {
        Connector {
            port: 8080,
            protocol: "HTTP/1.1".to_string(),
            connection_timeout: 20000,
            redirect_port: 8433,
        }
    }
}

/// <Service name="Catalina">
///     <Connector ... />
///     <Engine ... />
/// </Service>
#[derive(Deserialize, Serialize, Debug)]
pub struct Service {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "Connector")]
    connector: Connector,
    #[serde(rename = "Engine")]
    engine: Engine,
}

impl Default for Service {
    fn default() -> Self {
        Service {
            name: "Catalina".to_string(),
            connector: Connector::default(),
            engine: Engine::default(),
        }
    }
}

/// <Resource name="UserDatabase"
///           auth="Container"
///           type="org.apache.catalina.UserDatabase"
///           description="User database that can be updated and saved"
///           factory="org.apache.catalina.users.MemoryUserDatabaseFactory"
///           pathname="conf/tomcat-users.xml" />
#[derive(Deserialize, Serialize, Debug)]
pub struct Resource {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@auth")]
    auth: String,
    #[serde(rename = "@type")]
    classify: String,
    #[serde(rename = "@description")]
    description: String,
    #[serde(rename = "@factory")]
    factory: String,
    #[serde(rename = "@pathname")]
    pathname: String,
}

impl Default for Resource {
    fn default() -> Self {
        Resource {
            name: "UserDatabase".to_string(),
            auth: "Container".to_string(),
            classify: "org.apache.catalina.UserDatabase".to_string(),
            description: "User database that can be updated and saved".to_string(),
            factory: "org.apache.catalina.users.MemoryUserDatabaseFactory".to_string(),
            pathname: "conf/tomcat-users.xml".to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GlobalNamingResources {
    #[serde(rename = "Resource")]
    resources: Vec<Resource>,
}

/// <GlobalNamingResources>
///     <Resource ... />
/// </GlobalNamingResources>
impl Default for GlobalNamingResources {
    fn default() -> Self {
        GlobalNamingResources {
            resources: vec![Resource::default()],
        }
    }
}

/// <Listener className="org.apache.catalina.startup.VersionLoggerListener" />
/// <Listener className="org.apache.catalina.core.AprLifecycleListener" SSLEngine="on" />
/// <Listener className="org.apache.catalina.core.JreMemoryLeakPreventionListener" />
/// <Listener className="org.apache.catalina.mbeans.GlobalResourcesLifecycleListener" />
/// <Listener className="org.apache.catalina.core.ThreadLocalLeakPreventionListener" />
#[derive(Deserialize, Serialize, Debug)]
pub struct Listener {
    #[serde(rename = "@className")]
    class_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AprLifecycleListener {
    #[serde(rename = "@className")]
    class_name: String,
    #[serde(rename = "@SSLEngine")]
    ssl_engine: String,
}

impl Listener {
    pub fn new(cn: &str) -> Self {
        Listener {
            class_name: cn.to_string(),
        }
    }
}

impl Default for AprLifecycleListener {
    fn default() -> Self {
        AprLifecycleListener {
            class_name: "org.apache.catalina.core.AprLifecycleListener".to_string(),
            ssl_engine: "on".to_string(),
        }
    }
}

/// <Server port="8005" shutdown="SHUTDOWN">
///     <Listener ... />
///     <GlobalNamingResources />
///     <Service ... />
/// </Server>
#[derive(Deserialize, Serialize, Debug)]
pub struct Server {
    #[serde(rename = "@port")]
    port: i32,
    #[serde(rename = "@shutdown")]
    shutdown: String,
    #[serde(rename = "Listener")]
    listener: AprLifecycleListener,
    #[serde(rename = "Listener")]
    listeners: Vec<Listener>,
    #[serde(rename = "GlobalNamingResources")]
    resource: GlobalNamingResources,
    #[serde(rename = "Service")]
    service: Service,
}

impl Server {
    pub fn new_tomcat7() -> Self {
        Self {
            port: 8005,
            shutdown: "SHUTDOWN".to_string(),
            resource: GlobalNamingResources::default(),
            service: Service::default(),
            listener: AprLifecycleListener::default(),
            listeners: vec![
                Listener::new("org.apache.catalina.startup.VersionLoggerListener"),
                Listener::new("org.apache.catalina.core.JasperListener"),
                Listener::new("org.apache.catalina.core.JreMemoryLeakPreventionListener"),
                Listener::new("org.apache.catalina.mbeans.GlobalResourcesLifecycleListener"),
                Listener::new("org.apache.catalina.core.ThreadLocalLeakPreventionListener"),
            ],
        }
    }
    fn new_tomcat8() -> Self {
        Self {
            port: 8005,
            shutdown: "SHUTDOWN".to_string(),
            resource: GlobalNamingResources::default(),
            service: Service::default(),
            listener: AprLifecycleListener::default(),
            listeners: vec![
                Listener::new("org.apache.catalina.startup.VersionLoggerListener"),
                Listener::new("org.apache.catalina.core.JreMemoryLeakPreventionListener"),
                Listener::new("org.apache.catalina.mbeans.GlobalResourcesLifecycleListener"),
                Listener::new("org.apache.catalina.core.ThreadLocalLeakPreventionListener"),
            ],
        }
    }
}

pub struct ServerXml {
    server: Server,
}

impl ServerXml {
    pub fn new_tomcat7() -> Self {
        ServerXml {
            server: Server::new_tomcat7(),
        }
    }

    pub fn new_tomcat8() -> Self {
        ServerXml {
            server: Server::new_tomcat8(),
        }
    }

    pub fn server_port(&mut self, port: i32) {
        self.server.port = port;
    }

    pub fn http_port(&mut self, port: i32) {
        self.server.service.connector.port = port;
    }

    pub fn add_context(&mut self, path: &str, doc: &str) {
        self.server
            .service
            .engine
            .host
            .contexts
            .push(Context::new(path, doc));
    }
}

impl ToString for ServerXml {
    fn to_string(&self) -> String {
        match quick_xml::se::to_string(&self.server) {
            Ok(val) => val,
            Err(e) => {
                if log_enabled!(Level::Error) {
                    error!("{}", e.to_string());
                }
                exit(0);
            }
        }
    }
}
