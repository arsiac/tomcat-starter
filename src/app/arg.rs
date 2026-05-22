use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

use crate::app;

#[derive(Parser, Debug)]
#[command(version = app::VERSION, disable_version_flag = true, disable_help_flag = false)]
pub struct Argument {
    #[arg(short = 'c', long = "config", help = "指定配置文件路径")]
    pub config: Option<String>,

    #[command(subcommand)]
    pub action: Action,
}

impl Argument {
    pub fn config_path(&self) -> Option<PathBuf> {
        self.config.as_ref().map(PathBuf::from)
    }
}

#[derive(Debug, Subcommand)]
pub enum Action {
    #[command(about = "Run web project")]
    Run(ActionRun),
    #[command(about = "Clear project's cache and logs")]
    Clean(ActionClean),
    #[command(about = "List projects or project's items")]
    List(ActionList),
    #[command(about = "Tms config")]
    Config,
    #[command(about = "Print version")]
    Version,
}

#[derive(Debug, Args)]
pub struct ActionList {
    #[arg(help = "Name of the project")]
    pub project: Option<String>,
}

#[derive(Debug, Args)]
pub struct ActionClean {
    #[arg(help = "Name of the project that needs to be cleaned")]
    pub project: Option<String>,

    #[arg(short, long, default_value_t = false)]
    #[arg(help = "Clean up all the projects")]
    pub all: bool,
}

#[derive(Debug, Args)]
pub struct ActionRun {
    #[arg(help = "Name of the project that needs to be run")]
    pub project: String,

    #[arg(short, long, default_value_t = false)]
    #[arg(help = "Run all items of the project")]
    pub all_items: bool,

    #[arg(short, long = "item")]
    #[arg(help = "Run specified items of the project")]
    pub items: Vec<String>,

    #[arg(short, long, default_value_t = false)]
    #[arg(help = "Run project in debug mode")]
    pub debug: bool,

    #[arg(long, help = "Tomcat HTTP port")]
    pub http_port: Option<u32>,

    #[arg(long, help = "Tomcat server port")]
    pub server_port: Option<u32>,

    #[arg(long, help = "Tomcat JPDA port")]
    pub jpda_port: Option<u32>,
}

#[derive(Debug, Args)]
pub struct  ActionConfig {
    #[arg(long)]
    #[arg(help = "Print example config")]
    pub example: bool
}
