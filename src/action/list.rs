use crate::action::Actions;
use crate::argument::TmsArgActionList;
use crate::config::TmsConfiguration;
use crate::config::{Project, ProjectItem};
use log::error;
use prettytable::{row, Table};
use std::collections::HashMap;

pub struct ListAction {
    argument: Box<TmsArgActionList>,
    configuration: Box<TmsConfiguration>,
}

impl ListAction {
    pub fn new(argument: Box<TmsArgActionList>, configuration: Box<TmsConfiguration>) -> Self {
        Self {
            argument,
            configuration,
        }
    }
}

impl Actions for ListAction {
    fn process(&self) -> bool {
        if self.argument.project {
            list_projects(&self.configuration.projects);
            return true;
        }

        match self.argument.item {
            None => {
                error!("Missing --project(-p) or --item(-i) <PROJECT NAME>.");
                return false;
            }
            Some(ref name) => list_items(&self.configuration, name.as_str()),
        }
    }
}

fn list_projects(project_map: &HashMap<String, Project>) {
    if project_map.is_empty() {
        println!("No projects.");
        return;
    }

    let mut projects: Vec<&Project> = project_map.values().collect();
    projects.sort_by(|a, b| a.name.cmp(&b.name));

    let mut table = Table::new();
    table.add_row(row!["Project", "Alias", "Description"]);
    for project in projects {
        let mut aliases = String::new();
        let mut alias_count = 0;
        for i_name in &project.alias {
            if alias_count == 0 {
                aliases = i_name.clone();
            } else {
                aliases = format!("{}, {}", &aliases, i_name);
            }
            alias_count = alias_count + 1;
        }
        let description = match &project.description {
            None => String::from(" - "),
            Some(val) => val.clone(),
        };
        table.add_row(row![project.name.clone(), aliases, description]);
    }
    table.printstd();
}

fn list_items(config: &TmsConfiguration, name: &str) -> bool {
    match config.get_project(name) {
        None => {
            error!("Project {} not exists.", name);
            false
        }
        Some(project) => {
            let mut table = Table::new();
            table.add_row(row!["Project", "Item", "Alias"]);
            let mut items: Vec<&ProjectItem> = project.items.values().collect();
            items.sort_by(|a, b| a.name.cmp(&b.name));

            for item in items {
                let mut aliases = String::new();
                let mut alias_count = 0;
                for i_name in &item.alias {
                    if alias_count == 0 {
                        aliases = i_name.clone();
                    } else {
                        aliases = format!("{}, {}", &aliases, i_name);
                    }
                    alias_count = alias_count + 1;
                }
                table.add_row(row![project.name.clone(), item.name.clone(), aliases]);
            }
            table.printstd();
            true
        }
    }
}
