use crate::env_ini::ini::{Ini, IniSection};
use log::{log_enabled, trace, Level};
use regex::Regex;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use crate::env_ini::lines;

const ENV_REGEX: &str = r"\$\{([a-zA-Z][a-zA-Z0-9_\.]+)\}";

#[derive(PartialEq)]
enum IniAnalyzeState {
    Ready,
    Section,
    Key,
    Value(String),
    Comment,
}

pub struct IniLoader {
    resolve_env: bool,
}

impl Display for IniAnalyzeState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IniAnalyzeState::Ready => write!(f, "Ready"),
            IniAnalyzeState::Section => write!(f, "Section"),
            IniAnalyzeState::Key => write!(f, "Key"),
            IniAnalyzeState::Value(s) => write!(f, "Value({})", s),
            IniAnalyzeState::Comment => write!(f, "Comment"),
        }
    }
}

impl IniLoader {
    /// 创建 IniLoader
    /// `resolve_env`: 是否解析 value 中的环境变量
    pub fn new(resolve_env: bool) -> Self {
        IniLoader {
            resolve_env
        }
    }

    pub fn load_path(&mut self, p: &Path) -> Result<Ini, String> {
        if !p.exists() {
            return Err(format!("File not exists: {}", p.to_str().unwrap()));
        }
        match File::open(p) {
            Err(e) => Err(e.to_string()),
            Ok(file) => self.load_file(&file),
        }
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
    fn load_file(&mut self, file: &File) -> Result<Ini, String> {
        let mut ini = Ini::new();
        let reader = BufReader::new(file);

        let mut state = IniAnalyzeState::Ready;
        let mut section = IniSection::default();
        for line in reader.lines() {
            if let Err(e) = line {
                return Err(e.to_string());
            }
            let line = line.unwrap();
            let trim_line = line.trim();

            trace!("Analyse line: {}", line);
            let mut safe_loop = 0;
            loop {
                safe_loop = safe_loop + 1;
                if safe_loop > 3 {
                    panic!("Loop too mach.");
                }
                trace!("State: {}", &state);
                match state {
                    IniAnalyzeState::Ready => {
                        if lines::is_blank(trim_line) {
                            break;
                        }
                        if lines::is_comment(trim_line) {
                            state = IniAnalyzeState::Comment;
                        } else if lines::is_section(trim_line) {
                            state = IniAnalyzeState::Section;
                        } else {
                            state = IniAnalyzeState::Key;
                        }
                    }
                    IniAnalyzeState::Comment => {
                        trace!("Skip comment: {}", line);
                        state = IniAnalyzeState::Ready;
                        break;
                    }
                    IniAnalyzeState::Section => {
                        // Save old section
                        trace!("Save old section: {}", &section.name);

                        ini.put(section);
                        // Get new section
                        let section_name = (&trim_line[1..trim_line.len() - 1]).trim();
                        trace!("Get new section: {}", section_name);
                        section = ini.get_or_create(section_name).clone();
                        state = IniAnalyzeState::Ready;
                        break;
                    }
                    IniAnalyzeState::Key => {
                        if lines::is_blank(trim_line) {
                            break;
                        }
                        match line.find("=") {
                            None => {
                                trace!("key: '{}', value: None", &line);
                                section.set(&line, None);
                                state = IniAnalyzeState::Ready;
                                break;
                            }
                            Some(index) => {
                                let key = (&trim_line[..index]).trim();
                                let value = (&trim_line[index + 1..]).trim();

                                // 替换环境变量
                                match self.resolve_value(value) {
                                    None => {
                                        trace!("key: {}, value: {}", key, value);
                                        section.set(key, Some(value));
                                    }
                                    Some(val) => {
                                        let val = val.as_str();
                                        trace!("key: {}, value: {}", key, val);
                                        section.set(key, Some(val));
                                    }
                                }

                                // 解析状态更新
                                if lines::is_value_end(trim_line) {
                                    state = IniAnalyzeState::Ready;
                                } else {
                                    trace!("value is not end: {}", line);
                                    state = IniAnalyzeState::Value(key.to_string());
                                }
                                break;
                            }
                        }
                    }
                    IniAnalyzeState::Value(ref key) => {
                        if lines::is_comment(trim_line) {
                            break;
                        }

                        // 如果下一行的开头不是空白，或者行为空，代表值已经结束
                        if !lines::start_with_blank(&line) || lines::is_blank(trim_line) {
                            state = IniAnalyzeState::Ready;
                        } else {
                            let value = section.get(key).unwrap();
                            let value = &value[..value.len() - 1];
                            let value = format!("{}{}", value, trim_line);

                            // 替换环境变量
                            match self.resolve_value(value.as_str()) {
                                None => {
                                    trace!("key: {}, value: {}", key, value);
                                    section.set(key, Some(value.as_str()));
                                }
                                Some(val) => {
                                    let val = val.as_str();
                                    trace!("key: {}, value: {}", key, val);
                                    section.set(key, Some(val));
                                }
                            }

                            // 值结束则清除 key 缓存并更新解析状态
                            if lines::is_value_end(trim_line) {
                                state = IniAnalyzeState::Ready;
                            }
                            break;
                        }
                    }
                }
            }
        }
        // Save last section

        trace!("Save last section: {}", &section.name);
        ini.put(section);

        trace!("ini: {:?}", &ini);
        Ok(ini)
    }
}
