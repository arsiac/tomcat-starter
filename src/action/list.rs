use prettytable::{row, Table};

use crate::config::TmsConfig;
use crate::app::AppError;

pub fn list_projects(config: &TmsConfig) {
    let mut table = Table::new();
    table.add_row(row!["Project", "Alias", "Description"]);
    for project in &config.projects {
        let name = project.name.as_str();
        let alias = match &project.alias {
            Some(alias) => alias.as_str(),
            None => "",
        };
        let description = match &project.description {
            Some(description) => description.as_str(),
            None => "",
        };
        table.add_row(row![name, alias, description]);
    }
    table.printstd();
}

pub fn list_project_items(config: &TmsConfig, project_name: &str) -> Result<(), AppError> {
    let mut table = Table::new();
    table.add_row(row!["Project", "Item", "Alias", "Context Path"]);
    let project = super::get_project(config, project_name)?;
    let project_name = project.name.as_str();
    for item in &project.items {
        let name = item.name.as_str();
        let alias = match &item.alias {
            Some(alias) => alias.as_str(),
            None => "",
        };
        let context_path = item.context_path.as_str();
        table.add_row(row![project_name, name, alias, context_path]);
    }
    table.printstd();
    Ok(())
}
