use std::fs;
use std::path::Path;
use log::{debug, error, info, Level, log_enabled, trace};
use crate::action::Actions;
use crate::argument::{CleanTypeEnum, TmsArgActionClean};
use crate::config::{TmsConfiguration};

pub struct CleanAction {
    argument: Box<TmsArgActionClean>,
    configuration: Box<TmsConfiguration>,
}

impl CleanAction {
    pub fn new(argument: Box<TmsArgActionClean>, configuration: Box<TmsConfiguration>) -> Self {
        Self {
            argument,
            configuration,
        }
    }

    fn clean_all_projects(&self, clean_type: &CleanTypeEnum) -> bool {
        let tc = &self.configuration.common;
        if !tc.cache_dir.exists() {
            if log_enabled!(Level::Warn) {
                error!("Cache directory not exists: {}", tc.cache_dir.to_str().unwrap());
            }
            return false;
        }

        if clean_type == &CleanTypeEnum::All {
            Self::remove_items(tc.cache_dir.as_path());
            return true;
        }

        if let Ok(read_dir) = fs::read_dir(tc.cache_dir.as_path()) {
            for entry in read_dir {
                match entry {
                    Ok(ref entry) => {
                        if !entry.file_type().unwrap().is_dir() {
                            info!("{}: skip", entry.file_name().to_str().unwrap());
                        } else {
                            info!("Clean project: {}", entry.file_name().to_str().unwrap());
                            Self::clean(clean_type, entry.path().as_path());
                        }
                    }
                    Err(e) => {
                        error!("{}", e.to_string());
                    }
                }
            }
        }
        true
    }

    fn clean_project(&self, name: &str, clean_type: &CleanTypeEnum) -> bool {
        let project_cache = self.configuration.common.cache_dir.join(name);
        if !project_cache.exists() {
            return true;
        }
        Self::clean(clean_type, project_cache.as_path())
    }

    fn clean(clean_type: &CleanTypeEnum, cache_path: &Path) -> bool {
        match clean_type {
            CleanTypeEnum::Cache => {
                let path = cache_path.join("webapps");
                Self::remove_items(path.as_path());
            }
            CleanTypeEnum::Log => {
                let path = cache_path.join("logs");
                Self::remove_items(path.as_path());
            }
            CleanTypeEnum::All => {
                match fs::read_dir(cache_path) {
                    Ok(dir) => {
                        for entry in dir {
                            match entry {
                                Ok(entry) => {
                                    Self::remove_dir(entry.path().as_path());
                                }
                                Err(e) => {
                                    error!("{}", e);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("{}", e);
                    }
                }
            }
        }
        true
    }

    fn remove_dir(path: &Path) -> bool {
        if log_enabled!(Level::Debug) {
            debug!("Remove directory: {}", path.to_str().unwrap());
        }
        if let Err(e) = fs::remove_dir_all(path) {
            error!("Remove {} failed: {}", path.to_str().unwrap(), e.to_string());
            return false;
        }
        true
    }

    fn remove_file(path: &Path) -> bool {
        if log_enabled!(Level::Debug) {
            debug!("Remove file: {}", path.to_str().unwrap());
        }
        if let Err(e) = fs::remove_file(path) {
            error!("Remove {} failed: {}", path.to_str().unwrap(), e.to_string());
            return false;
        }
        true
    }

    fn remove_items(path: &Path) -> bool {
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
                                Self::remove_dir(entry.path().as_path());
                            } else {
                                Self::remove_file(entry.path().as_path());
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
}

impl Actions for CleanAction {
    fn process(&self) -> bool {
        if self.argument.all_project {
            return self.clean_all_projects(&self.argument.target);
        }

        match self.argument.project {
            None => {
                error!("Missing --project(-p) <PROJECT> or --all.");
                false
            }
            Some(ref name) => {
                match self.configuration.get_project(name.as_str()) {
                    None => {
                        error!("Project not exists: {}", name);
                        false
                    }
                    Some(p) => {
                        trace!("Clean project: {}", &p.name);
                        self.clean_project(&p.name, &self.argument.target)
                    }
                }
            }
        }
    }
}
