use std::path::{Path, PathBuf};

/// 当前系统是否是 Windows
pub fn is_windows() -> bool {
    cfg!(target_family = "windows")
}

/// 程序所在文件夹
pub fn get_program_folder() -> PathBuf {
    let program_path = std::env::current_exe().unwrap();
    let program_folder = program_path.parent().unwrap();
    program_folder.to_path_buf()
}

/// 获取 catalina 可执行文件路径
pub fn get_catalina(catalina_home: &Path) -> PathBuf {
    let bin_folder = PathBuf::from(catalina_home).join("bin");
    if is_windows() {
        bin_folder.join("catalina.bat")
    } else {
        bin_folder.join("catalina.sh")
    }
}

/// 获取 java 可执行文件
pub fn get_java(java_home: &Path) -> PathBuf {
    let bin_folder = java_home.join("bin");
    if is_windows() {
        bin_folder.join("java.exe")
    } else {
        bin_folder.join("java")
    }
}
