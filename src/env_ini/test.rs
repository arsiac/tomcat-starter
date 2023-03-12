#[cfg(test)]
mod tests {
    use crate::env_ini::loader::IniLoader;
    use log::LevelFilter;
    use std::env;

    fn init() {
        let _ = env_logger::builder()
            .is_test(true)
            .format_timestamp(None)
            .filter_level(LevelFilter::Trace)
            .try_init();
    }

    #[test]
    fn test_ini_load() {
        init();
        let file_path = env::current_dir()
            .unwrap()
            .join("config")
            .join("config.ini");
        println!("file: {}", file_path.to_str().unwrap());
        let mut loader = IniLoader::new(true);
        let ini_res = loader.load_path(file_path.as_path());
        if let Err(e) = ini_res {
            panic!("{}", e);
        }
        let ini = ini_res.unwrap();
        let section = ini.get("runtime");
        assert!(section.is_some());
        let section = section.unwrap();
        let java_home = section.get("java_home");
        let env_java_home = env::var("JAVA_HOME");
        match env_java_home {
            Err(_) => {
                assert_eq!(java_home, None);
            }
            Ok(val) => {
                assert_eq!(java_home, Some(&val))
            }
        }
    }
}