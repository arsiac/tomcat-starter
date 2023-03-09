# TMS

使用 `Tomcat` 运行 web 项目

## 使用

### 运行项目

``` shell
# 选择部分项目
tms run --project <project> --item <item-1> --item <item-2> --item <item-n>

# 运行所有子项
tms run --project <project> --all-item
```

### 在新窗口打开 `Windows`

``` shell
tms run --project <project> --item <item-1> --item <item-2> --separate
```

### 打开 `JPDA` 调试

``` shell
tms run --project <project> --item <item-1> --item <item-2> --debug
```

### 清理缓存或日志

``` shell
# 清理项目缓存
tms clean --target cache --project <project>
# 清理项目日志
tms clean --target log --project <project>
# 清理全部
tms clean --target all --project <project>  
```

### 清理所有项目

``` shell
tms clean --target all --all--project
```

## 配置文件

配置文件与程序在同一目录中

### 配置样例

``` ini
[common]
log_level = info
; 默认缓存在程序所在目录
; cache_dir =

[runtime]
java_home = ${JAVA_HOME}
java_opts = -XX:+HeapDumpOnOutOfMemoryError \
            -XX:-OmitStackTraceInFastThrow
; 是否记录 Tomcat 日志文件
enable_logfile = false
catalina_home = ${CATALINA_HOME}
http_port = 8080
server_port = 8005
jpda_port = 5005

[project]
; 相对路径或者绝对路径
include = project/demo.ini

; demo 项目
[project "demo"]
alias = d
; java_home = 
enable_logfile = true
catalina_home = ${CATALINA_HOME}
http_port = 8080
server_port = 8005
jpda_port = 5005
java_opts = -Dproject.example=demo
jpda_port = 9888

item.app1.alias = a1
item.app1.context_path = /app-one
item.app1.source_path = /path/to/app1

item.app2.alias = a2
item.app2.context_path = /app2
item.app2.source_path = /path/to/app2
```