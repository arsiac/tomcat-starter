# TMS - Tomcat 启动器 CLI

## 构建与运行

```bash
cargo build              # 构建
cargo run -- <args>      # 运行
cargo build --release    # 发布构建
```

## 命令 (clap CLI, binary: `tms`)

| 命令 | 说明 |
|------|------|
| `tms config` | 打印示例配置（通过 rust-embed 内嵌 `config-sample.toml`） |
| `tms list [project]` | 列出项目/子项 (prettytable-rs 表格) |
| `tms run <project> -a` | 运行项目的所有子项 |
| `tms run <project> -i <item> [-i <item>...]` | 运行指定子项 |
| `tms run <project> -a -d` | 以 JPDA 调试模式运行 |
| `tms run <project> -a --http-port 8081 --server-port 8005 --jpda-port 8000` | 指定端口覆盖 |
| `tms clean <project>` | 清除项目缓存 |
| `tms clean -a` | 清除所有项目缓存 |

## 配置 (TOML)

- 配置文件: `{config_dir}/config.toml`，可通过 `-c, --config` 指定自定义路径
- config_dir = `dirs::config_dir()/tms`（用户配置目录，debug/release 一致）
- `tms -c <路径>` 测试时使用项目内配置，例如 `tms -c config/config.toml list`
- 缓存目录: `dirs::cache_dir()/tms`（用户缓存目录，debug/release 一致）
- `include = ["path.toml"]` 支持导入其他 TOML，**仅合并 `[[project]]` 条目**，路径相对于配置文件所在目录
- `[default.java]` 和 `[default.tomcat]` 是全局默认值（可被项目级 `[project.runtime.*]` 覆盖）
- `java_home`/`tomcat_home` 未配置时回退到环境变量 `JAVA_HOME`/`CATALINA_HOME`
- 端口默认值: http=8080, server=8005, jpda=8000
- 端口有效范围: 1024-65535

## 架构

```
src/main.rs              入口 → config::init() → app::run()
src/config/              TOML 解析、域模型、配置加载（含 include 合并）
src/app/{arg,logger,constant,util,error}   CLI 参数、日志、常量、工具函数
src/action/{run,list,clean}  三个子命令实现
```

## 核心行为

- `run` 命令的工作流: 将 Tomcat conf 复制到缓存目录 → 修改 server.xml (通过 xml-doc 插入 Context 元素，docBase=war路径) → 设置环境变量 (JAVA_HOME, CATALINA_HOME/BASE/OPTS, TITLE) → spawn catalina.bat/sh
- `clean` 命令删除 `{cache_dir}/{project_name}/` 整个目录
- 日志: simplelog `TermLogger`，默认 `info` 级别，**不显示时间戳/位置/线程**

## 平台差异

- Windows: `bin/catalina.bat`，`bin/java.exe`
- Unix: `bin/catalina.sh`，`bin/java`
- 在 `src/app/constant.rs:8-16` 中通过 `#[cfg(target_os = "windows")]` 区分
