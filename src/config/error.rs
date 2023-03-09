#[derive(std::fmt::Debug)]
pub struct ConfigurationError {
    msg: String,
}

impl std::fmt::Display for ConfigurationError {
    /// 展示异常信息
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl From<&str> for ConfigurationError {
    fn from(s: &str) -> Self {
        ConfigurationError {
            msg: String::from(s),
        }
    }
}

impl From<String> for ConfigurationError {
    /// 从字符串构建异常
    /// ```
    /// ConfigurationError::from(e.to_string())
    /// ```
    fn from(msg: String) -> Self {
        ConfigurationError { msg }
    }
}
