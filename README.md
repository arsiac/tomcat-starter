# TMS

使用 `Tomcat` 运行 web 项目

## 使用

### 生成配置文件
``` shell
tms config > config.toml
```

### 运行项目

**选择部分项目运行**

``` shell
tms run <project> -i <item-1> -i <item-2> -i <item-n>
```
**运行所有子项**

```shell
tms run <project> -a
```

### 打开 `JPDA` 调试

``` shell
tms run  <project> -a -d
```

### 清理缓存和日志

**清理指定项目**

``` shell
tms clean <project>
```

**清理所有项目**

```shell
tms clean -a
```
