use std::collections::HashMap;

pub trait ValueGetter<R> {
    fn get() -> R;
}

#[derive(Debug)]
pub struct IniSection {
    pub name: String,
    config_map: HashMap<String, Option<String>>,
}

#[derive(Debug)]
pub struct Ini {
    section_map: HashMap<String, IniSection>,
}

impl IniSection {
    /// 创建 section
    pub fn new(name: &str) -> Self {
        IniSection {
            name: String::from(name),
            config_map: HashMap::new(),
        }
    }

    /// 是否存在配置 key
    // pub fn contains(&self, key: &str) -> bool {
    //     self.config_map.contains_key(key)
    // }

    /// 获取 key 对应的 value
    pub fn get(&self, key: &str) -> Option<&String> {
        match self.config_map.get(key) {
            None => None,
            Some(ops) => match ops {
                None => None,
                Some(val) => Some(val),
            },
        }
    }

    /// 设置配置键值对，若 value 为空，则认为未配置
    pub fn set(&mut self, key: &str, value: Option<&str>) {
        let key = String::from(key);
        match value {
            Some(value) => {
                if value.is_empty() {
                    self.config_map.insert(key, None);
                } else {
                    self.config_map.insert(key, Some(String::from(value)));
                }
            }
            None => {
                self.config_map.insert(key, None);
            }
        }
    }

    /// 获取键值对迭代器
    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, String, Option<String>> {
        self.config_map.iter()
    }
}

impl Default for IniSection {
    /// 创建默认分组
    fn default() -> Self {
        Self::new("default")
    }
}

impl Clone for IniSection {
    /// 复制 section
    fn clone(&self) -> Self {
        let mut map = HashMap::new();
        for (key, value) in &self.config_map {
            map.insert(key.clone(), value.clone());
        }
        IniSection {
            name: self.name.clone(),
            config_map: map,
        }
    }
}

impl Ini {
    /// 创建 Ini 空实例
    pub fn new() -> Self {
        Ini {
            section_map: HashMap::new(),
        }
    }

    /// 是否存在对应 section
    pub fn contains(&self, section_name: &str) -> bool {
        self.section_map.contains_key(section_name)
    }

    /// 获取 section
    pub fn get(&self, section_name: &str) -> Option<&IniSection> {
        self.section_map.get(section_name)
    }

    /// 创建 section，如果已存在则返回 None
    pub fn create(&mut self, section_name: &str) {
        if !self.contains(section_name) {
            let section = IniSection::new(section_name);
            self.section_map.insert(section.name.clone(), section);
        }
    }

    /// 存储新的 section
    pub fn put(&mut self, section: IniSection) {
        self.section_map.insert(section.name.clone(), section);
    }

    /// 获取或新建 section
    pub fn get_or_create(&mut self, section_name: &str) -> &IniSection {
        self.create(section_name);
        self.get(section_name).unwrap()
    }

    /// 获取迭代器
    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, String, IniSection> {
        self.section_map.iter()
    }
}
