use std::path::PathBuf;

pub use domain::*;
pub use inspect::init;

mod domain;
mod inspect;
mod file;

pub fn get_exe_directory() -> PathBuf {
    match std::env::current_exe() {
        Ok(path) => match path.parent() {
            Some(path) => path.to_path_buf(),
            None => panic!("Exe directory not exists"),
        },
        Err(e) => panic!("Failed to get exe directory: {}", e),
    }
}

pub fn get_config_dir() -> PathBuf {
    #[cfg(debug_assertions)]
    match std::env::var("CARGO_MANIFEST_DIR") {
        Ok(path) => PathBuf::from(path).join("config"),
        Err(e) => panic!("Failed to get cargo manifest dir: {}", e),
    }

    #[cfg(not(debug_assertions))]
    get_exe_directory()
}

pub fn get_cache_dir() -> PathBuf {
    get_exe_directory().join("cache")
}
