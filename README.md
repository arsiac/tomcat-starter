# tomcat-starter

Run or debug web project by Tomcat.

## Usage

``` shell
tms -p xxx_project -w war1 -w war2 -w war3 -n
```

- `--project`, `-p`: project name

- `--web-document`, `-w`: web document name

- `--debug-mode`, `-d`: open Tomcat debug mode.(jpda)

- `--new-windows`, `-n`: open in new window

- `--config-template`: configuration file template

- `--clean`: clean cache directory

 - `--version`: show Tomcat Starter version

## Configuration

### Localtion

``` ini
Linux: ${HOME}/.tms/config.ini
Windows: $env:USERPROFILE#.tms#config.ini
```

### Template

``` ini
; ${HOME}/.tms/config.ini
[global]
; debug, info, warn, error and close
log_level = warn
java_home = $env:JAVA_HOME
java_opts = $env:JAVA_OPTS -XX:+HeapDumpOnOutOfMemoryError -XX:-OmitStackTraceInFastThrow
tomcat = $env:CATALINA_HOME
http_port = 8080
server_port = 8005
jpda_port = 5005
cache_dir = $env:USERPROFILE\.tms

[project "web"]
java_home = xxx
java_opts = xxx
tomcat = xxx
http_port = xxx
server_port = xxx
jpda_port = xxx

[web "web"]
war1 = war1|/path/to/xxx.war
xxx = xxx_context|/path/to/xxx.war
```

### Detail

#### `[global]`

- `log_level`: 日志级别

- `cache_dir`: 缓存目录

#### `[global]`, `[project "name"]`

- `java_home`: JDK path

- `java_opts`: JVM parameters

- `tomcat`: Tomcat path(CATALINA_HOME)

- `http_port`: Tomcat HTTP port

- `server_port`: Tomcat Server port

- `jpda_port`: Tomcat debug port

#### `[web "name"]`

```ini
[doc_name]=[context-path]|[web document path]
war1 = war1-web|/path/to/web/document
```