use clap::Parser;

use crate::action;
use crate::config::TmsConfig;
pub use error::AppError;

pub mod error;
pub mod logger;
pub mod util;
pub mod constant;
pub mod arg;

pub const VERSION: &str = "0.2.0";

#[derive(rust_embed::Embed)]
#[folder = "config/"]
#[exclude = "config.toml"]
struct ExampleConfig;

pub fn run(config: &TmsConfig) -> Result<(), AppError> {
    let args = arg::Argument::parse();
    match args.action {
        arg::Action::Run(action) => {
            if !action.all_items && action.items.is_empty() {
                return Err(AppError::Argument("items is required".to_string()));
            }
            action::run::run_project(&action, config)?;
        }
        arg::Action::Clean(info) => match &info.project {
            None => {
                if info.all {
                    action::clean::clean_all_projects(config)?;
                } else {
                    return Err(AppError::Argument("project name is required".to_string()));
                }
            }
            Some(project) => {
                action::clean::clean_project(config, project)?;
            }
        },
        arg::Action::List(action) => match action.project {
            None => action::list::list_projects(config),
            Some(project) => action::list::list_project_items(config, &project)?,
        },
        arg::Action::Config => {
            let sample_file = ExampleConfig::get( "config-sample.toml").unwrap();
            println!("{}", std::str::from_utf8(sample_file.data.as_ref()).unwrap());
        }
        arg::Action::Version => {
            println!("tms version {}", VERSION);
        }
    }

    Ok(())
}
