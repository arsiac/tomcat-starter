use crate::env_ini::ini::{Ini, IniSection};
use log::{log_enabled, trace, Level};
use regex::Regex;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub trait IniStrLoad<T, E> {
    fn load(&mut self, s: &str) -> Result<&T, E>;
}

pub trait IniPathLoad<T, E> {
    fn load(&mut self, p: &Path) -> Result<&T, E>;
}

const ENV_REGEX: &str = r"\$\{([a-zA-Z][a-zA-Z0-9_\.]+)\}";

pub struct IniLoader {
    resolve_env: bool,
    ini: Ini,
}

#[derive(PartialEq)]
enum IniAnalyzeState {
    Ready,
    Section,
    Pair,
    Comment,
}

impl Display for IniAnalyzeState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IniAnalyzeState::Ready => write!(f, "Ready"),
            IniAnalyzeState::Section => write!(f, "Section"),
            IniAnalyzeState::Pair => write!(f, "Pair"),
            IniAnalyzeState::Comment => write!(f, "Comment"),
        }
    }
}

impl IniLoader {
    /// 创建 IniLoader
    /// `resolve_env`: 是否解析 value 中的环境变量
    pub fn new(resolve_env: bool) -> Self {
        IniLoader {
            resolve_env,
            ini: Ini::new(),
        }
    }

    /// 清空配置
    pub fn clear(&mut self) {
        if !self.ini.is_empty() {
            self.ini = Ini::new();
        }
    }

    /// 返回 ini 配置
    // pub fn get_ini(&self) -> &Ini {
    //     return &self.ini;
    // }

    fn is_comment(line: &str) -> bool {
        line.starts_with(";") || line.starts_with("#")
    }

    fn is_section(line: &str) -> bool {
        line.starts_with("[") && line.ends_with("]")
    }

    fn is_value_not_end(line: &str) -> bool {
        line.ends_with("\\")
    }

    fn resolve_value(&self, value: &str) -> Option<String> {
        if !self.resolve_env || value.find("${").is_none() {
            trace!("No env pattern: {}", value);
            return None;
        }

        trace!("Resolve env: {}", value);
        let regex = Regex::new(ENV_REGEX).unwrap();
        let mut result = String::from(value);
        for capture in regex.captures_iter(value) {
            let pattern = capture.get(0).unwrap();
            let content = capture.get(1).unwrap();
            if log_enabled!(Level::Trace) {
                trace!("Resolve pattern: {}", pattern.as_str());
            }
            match std::env::var(content.as_str()) {
                Ok(ref val) => {
                    if log_enabled!(Level::Trace) {
                        trace!("Env '{}': {}", content.as_str(), val.as_str());
                    }
                    if !val.is_empty() {
                        result = result.replace(pattern.as_str(), val);
                    }
                }
                Err(_) => {}
            }
        }
        if log_enabled!(Level::Trace) {
            trace!("Result : {}", result.as_str());
        }
        return Some(result);
    }

    /// 从文件加载 ini 配置
    fn load_ini_file(&mut self, file: &File) -> Result<&Ini, String> {
        self.clear();
        let mut current_section = IniSection::default();
        let mut state = IniAnalyzeState::Ready;
        let mut key_cache: Option<String> = None;
        for line in BufReader::new(file).lines() {
            if line.is_err() {
                return Err(line.err().unwrap().to_string());
            }
            let line = line.unwrap();
            let is_start_blank = line.starts_with(" ");
            let line = line.trim();
            if line.is_empty() && state != IniAnalyzeState::Pair {
                continue;
            }

            trace!("analyse line: {}", line);
            let mut safe_loop = 0;
            loop {
                safe_loop = safe_loop + 1;
                if safe_loop > 3 {
                    panic!("Loop too mach.");
                }
                trace!("state: {}", state);
                match state {
                    IniAnalyzeState::Ready => {
                        if Self::is_comment(&line) {
                            state = IniAnalyzeState::Comment;
                        } else if Self::is_section(line) {
                            state = IniAnalyzeState::Section;
                        } else {
                            state = IniAnalyzeState::Pair;
                        }
                    }
                    IniAnalyzeState::Comment => {
                        // Skip comments
                        trace!("Skip comment.");
                        state = IniAnalyzeState::Ready;
                        break;
                    }
                    IniAnalyzeState::Section => {
                        // Save old section
                        trace!("Save old section: {}", &current_section.name);
                        self.ini.put(current_section);

                        // Get new section
                        let section_name = (&line[1..line.len() - 1]).trim();
                        trace!("Get new section: {}", section_name);
                        current_section = self.ini.get_or_create(section_name).clone();
                        state = IniAnalyzeState::Ready;
                        break;
                    }
                    IniAnalyzeState::Pair => {
                        match key_cache {
                            None => {
                                trace!("No key cache.");
                                match line.find("=") {
                                    None => {
                                        trace!("key: '{}', value: None", line);
                                        current_section.set(line, None);
                                        state = IniAnalyzeState::Ready;
                                        break;
                                    }
                                    Some(index) => {
                                        let key = (&line[0..index]).trim();
                                        let value = (&line[index + 1..line.len()]).trim();

                                        // 替换环境变量
                                        match self.resolve_value(value) {
                                            None => {
                                                trace!("key: {}, value: {}", key, value);
                                                current_section.set(key, Some(value));
                                            }
                                            Some(val) => {
                                                let val = val.as_str();
                                                trace!("key: {}, value: {}", key, val);
                                                current_section.set(key, Some(val));
                                            }
                                        }

                                        // 解析状态更新
                                        if Self::is_value_not_end(value) {
                                            trace!("value ends with ' \\'. value is not end.");
                                            key_cache = Some(String::from(key));
                                        } else {
                                            state = IniAnalyzeState::Ready;
                                        }
                                        break;
                                    }
                                }
                            }

                            // 值还未结束
                            Some(ref key) => {
                                trace!("key cache: {}", key);
                                if Self::is_comment(line) {
                                    break;
                                }

                                // 如果下一行的开头不是空格，或者行为空，代表值已经结束
                                if !is_start_blank || line.is_empty() {
                                    key_cache = None;
                                    state = IniAnalyzeState::Ready;
                                } else {
                                    let value = current_section.get(key).unwrap();
                                    let value = &value.as_str()[0..value.len() - 1];
                                    let value = format!("{}{}", value, line);
                                    current_section.set(key, Some(value.as_str()));

                                    // 替换环境变量
                                    match self.resolve_value(value.as_str()) {
                                        None => {
                                            trace!("key: {}, value: {}", key, value);
                                            current_section.set(key, Some(value.as_str()));
                                        }
                                        Some(val) => {
                                            let val = val.as_str();
                                            trace!("key: {}, value: {}", key, val);
                                            current_section.set(key, Some(val));
                                        }
                                    }

                                    // 值结束则清除 key 缓存并更新解析状态
                                    if !Self::is_value_not_end(value.as_str()) {
                                        key_cache = None;
                                        state = IniAnalyzeState::Ready;
                                    }
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
        // Save last section

        trace!("Save last section: {}", &current_section.name);
        self.ini.put(current_section);

        Ok(&self.ini)
    }
}

impl IniStrLoad<Ini, String> for IniLoader {
    fn load(&mut self, s: &str) -> Result<&Ini, String> {
        match File::open(Path::new(s)) {
            Err(e) => Err(e.to_string()),
            Ok(file) => self.load_ini_file(&file),
        }
    }
}

impl IniPathLoad<Ini, String> for IniLoader {
    fn load(&mut self, p: &Path) -> Result<&Ini, String> {
        if !p.exists() {
            return Err(format!("File not exists: {}", p.to_str().unwrap()));
        }
        match File::open(p) {
            Err(e) => Err(e.to_string()),
            Ok(file) => self.load_ini_file(&file),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::env_ini::loader::{IniLoader, IniPathLoad};
    use log::LevelFilter;
    use std::env;

    fn init() {
        let _ = env_logger::builder()
            .is_test(true)
            .format_timestamp(None)
            .filter_level(LevelFilter::Trace)
            .try_init();
    }

    #[test]
    fn test_ini_load() {
        init();
        let file_path = env::current_dir()
            .unwrap()
            .join("config")
            .join("config.ini");
        println!("file: {}", file_path.to_str().unwrap());
        match IniLoader::new(true).load(file_path.as_path()) {
            Ok(ini) => {
                let section = ini.get("runtime");
                assert!(section.is_some());
                let section = section.unwrap();
                let java_home = section.get("java_home");
                let env_java_home = env::var("JAVA_HOME");
                match env_java_home {
                    Err(_) => {
                        assert_eq!(java_home, None);
                    }
                    Ok(val) => {
                        assert_eq!(java_home, Some(&val))
                    }
                }
            }
            Err(msg) => {
                panic!("{}", msg)
            }
        }
    }
}
