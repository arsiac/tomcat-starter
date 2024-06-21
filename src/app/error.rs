use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum AppError {
    System(String),
    Argument(String),
    Config(String),
    Action(String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::System(msg) => {
                write!(f, "system error: {}", msg)
            }
            AppError::Argument(msg) => {
                write!(f, "argument error: {}", msg)
            }
            AppError::Config(msg) => {
                write!(f, "configuration file error:{}", msg)
            }
            AppError::Action(msg) => {
                write!(f, "{}", msg)
            }
        }
    }
}
