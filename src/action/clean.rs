use crate::app;
use crate::config;

pub fn clean_all_projects(config: &config::TmsConfig) -> Result<(), app::AppError> {
    let cache_folder = config::get_cache_dir();
    for project in &config.projects {
        let project_cache = cache_folder.clone().join(&project.name);
        log::info!("Clean project cache: {}", project_cache.display());
        app::util::remove_dir_items(project_cache.as_path())?;
    }
    Ok(())
}

pub fn clean_project(config: &config::TmsConfig, project_name: &str) -> Result<(), app::AppError> {
    let project = super::get_project(config, project_name)?;
    let cache_folder = config::get_cache_dir();
    let project_cache = cache_folder.join(&project.name);
    log::info!("Clean project cache: {}", project_cache.display());
    app::util::remove_dir_items(project_cache.as_path())?;
    Ok(())
}