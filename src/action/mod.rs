use crate::config::{ProjectConfig, ProjectItemConfig, TmsConfig};
use crate::app::AppError;

pub mod list;
pub mod run;
pub mod clean;

fn filter_by_name(target_name: &str, name: &str, alias: Option<&String>) -> bool {
    if target_name == name {
        return true;
    }

    if let Some(alias) = alias {
        return target_name == alias.as_str();
    }

    false
}

fn get_project<'a>(
    config: &'a TmsConfig,
    project_name: &str,
) -> Result<&'a ProjectConfig, AppError> {
    let project = config
        .projects
        .iter()
        .find(|t| filter_by_name(project_name, t.name.as_str(), t.alias.as_ref()));
    match project {
        None => Err(AppError::Action(format!(
            "Project '{}' not found",
            project_name
        ))),
        Some(project) => Ok(project),
    }
}

fn get_project_item<'a>(
    config: &'a ProjectConfig,
    item_name: &str,
) -> Result<&'a ProjectItemConfig, AppError> {
    let project = config
        .items
        .iter()
        .find(|t| filter_by_name(item_name, t.name.as_str(), t.alias.as_ref()));
    match project {
        None => Err(AppError::Action(format!(
            "Item '{}' of project '{}' not found",
            item_name,
            config.name.as_str()
        ))),
        Some(item) => Ok(item),
    }
}
