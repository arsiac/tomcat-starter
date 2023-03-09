use log::{error, log_enabled, Level, debug, info};
use std::fs;
use std::path::Path;

pub fn create_dir(path: &Path) -> bool {
    match fs::create_dir(path) {
        Ok(_) => true,
        Err(e) => {
            if log_enabled!(Level::Error) {
                error!(
                    "Create directory failed: {}: {}",
                    path.to_str().unwrap(),
                    e.to_string()
                );
            }
            false
        }
    }
}

pub fn remove_dir(path: &Path) -> bool {
    if !path.exists() {
        return true;
    }
    if log_enabled!(Level::Debug) {
        debug!("Remove directory: {}", path.to_str().unwrap());
    }
    if let Err(e) = fs::remove_dir_all(path) {
        error!("Remove {} failed: {}", path.to_str().unwrap(), e.to_string());
        return false;
    }
    true
}

pub fn remove_file(path: &Path) -> bool {
    if !path.exists() {
        return true;
    }
    if log_enabled!(Level::Debug) {
        debug!("Remove file: {}", path.to_str().unwrap());
    }
    if let Err(e) = fs::remove_file(path) {
        error!("Remove {} failed: {}", path.to_str().unwrap(), e.to_string());
        return false;
    }
    true
}

pub fn remove_items(path: &Path) -> bool {
    if !path.exists() {
        return true;
    }
    if log_enabled!(Level::Info) {
        info!("Remove items: {}", path.to_str().unwrap());
    }
    match fs::read_dir(path) {
        Ok(dir) => {
            for entry in dir {
                match entry {
                    Ok(entry) => {
                        let file_type = entry.file_type().unwrap();
                        if file_type.is_dir() {
                            remove_dir(entry.path().as_path());
                        } else {
                            remove_file(entry.path().as_path());
                        }
                    }
                    Err(e) => {
                        error!("{}", e.to_string());
                    }
                }
            }
        }
        Err(e) => {
            error!("{}", e.to_string());
        }
    }
    true
}