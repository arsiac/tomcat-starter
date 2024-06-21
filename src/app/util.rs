use std::path::Path;

use crate::app::AppError;

pub fn is_valid_port(port: u32) -> bool {
    (1024..=65535).contains(&port)
}

pub fn check_port(name: &str, port: u32) -> Result<u32, AppError> {
    if !is_valid_port(port) {
        return Err(AppError::Action(format!(
            "{} must be between 1024 and 65535",
            name
        )));
    }
    Ok(port)
}

pub fn create_dirs(path: &Path) -> Result<(), AppError> {
    if path.exists() {
        return Ok(());
    }
    if let Err(e) = std::fs::create_dir_all(path) {
        return Err(AppError::System(e.to_string()));
    }

    Ok(())
}

pub fn remove_dir_items(path: &Path) -> Result<(), AppError> {
    if !path.exists() {
        return Ok(());
    }
    if let Err(e) = std::fs::remove_dir_all(path) {
        return Err(AppError::System(e.to_string()));
    }
    Ok(())
}
