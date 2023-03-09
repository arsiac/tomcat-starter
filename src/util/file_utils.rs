use log::{error, log_enabled, Level};
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
