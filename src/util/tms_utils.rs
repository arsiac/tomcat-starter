use std::env;
use std::path::PathBuf;

/// 获取tms所在目录
pub fn get_exe_folder() -> PathBuf {
    env::current_exe()
        .expect("Get executable file path failed.")
        .parent()
        .expect("Get Get executable file directory failed.")
        .to_path_buf()
}

/// 获取程序主目录
pub fn get_tms_home() -> PathBuf {
    match env::var("TMS_HOME") {
        Err(_) => get_exe_folder(),
        Ok(dir) => {
            if dir.is_empty() {
                get_exe_folder()
            } else {
                PathBuf::from(dir)
            }
        }
    }
}
