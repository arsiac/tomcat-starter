# Tomcat-Starter

## Build

### Linux

```shell
mkdir tomcat-starter/build
cd tomcat-starter/build
cmake ..
cmake --build .
```
### Windows

```shell
md tomcat-starter\build
cd tomcat-starter\build
cmake -G"Unix Makefiles" ..
cmake --build .
```

## Configuration

generated by `tms --config-template > config.ini`

``` ini
[common]
log_level = info
java_home = /path/to/jdk/
;global JVM options
jvm_opts = jvm_opts = -XX:+HeapDumpOnOutOfMemoryError -XX:-OmitStackTraceInFastThrow -Xmx2048m -Dfile.encoding=UTF-8
tomcat = /path/to/catalina/home
;default http port
http_port = 8080
;default shutdown port
shut_port = 8079

;web application name
[web-tst]
;web application http port.
;use default http port if not set
;http_port = 8082
;web application shutdown port.
;use default shutdown port if not set
;shut_port = 8081
;JVM option for this web application
jvm_opts = -Dhello=self
;sub web application context path
anyname_context = app-web
;sub web application war path
anyname_war = /pat/to/war
;anyname2_context = app2-web
;anyname2_war = /pat/to/war
```

## Usage
- start a web application

```shell
tms --app web-tst --sub-app anyname --sub-app anyname2
```
- start in new window

```shell
tms --app web-tst --sub-app anyname -n
```