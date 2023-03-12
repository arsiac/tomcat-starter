use clap::{Args, Parser, Subcommand, ValueEnum};
use std::fmt::{Display, Formatter};

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum CleanTypeEnum {
    Cache,
    Log,
    All,
}

#[derive(Args, Debug, Clone)]
#[command(long_about = "List project information")]
pub struct TmsArgActionList {
    /// 全部项目
    #[arg(short, long)]
    #[arg(help = "List all project information")]
    pub project: bool,

    /// 项目名称
    #[arg(short, long, required = false, id = "PROJECT")]
    #[arg(help = "List project information by project name")]
    pub item: Option<String>,
}

#[derive(Args, Debug, Clone)]
#[command(long_about = "Clean project cache or logs")]
pub struct TmsArgActionClean {
    #[arg(short, long, default_value_t = false)]
    #[arg(help = "All projects")]
    pub all_project: bool,
    #[arg(short, long, required = false)]
    #[arg(help = "Project name")]
    pub project: Option<String>,
    #[arg(short, long, value_enum, help = "Project's cache or logs")]
    pub target: CleanTypeEnum,
}

#[derive(Args, Debug, Clone)]
#[command(long_about = "Run project")]
pub struct TmsArgActionRun {
    #[arg(short, long)]
    #[arg(help = "Name of project to be run")]
    pub project: String,
    #[arg(short, long, default_value_t = false)]
    #[arg(help = "Run with all items")]
    pub all_item: bool,
    #[arg(short, long, required = false)]
    #[arg(help = "Name of item to be run")]
    pub item: Vec<String>,
    #[arg(short, long, default_value_t = false)]
    #[arg(help = "Start Tomcat under JPDA debugger")]
    pub debug: bool,
    #[arg(short, long, default_value_t = false)]
    #[arg(help = "Start Catalina in a separate window")]
    pub separate: bool,
}

#[derive(Subcommand, Debug)]
pub enum TmsAction {
    Run(TmsArgActionRun),
    Clean(TmsArgActionClean),
    List(TmsArgActionList),
}

#[derive(Parser, Debug)]
#[command(version = "0.1.0-beta")]
pub struct TmsArgument {
    #[command(subcommand)]
    pub action: TmsAction,
}

impl Display for CleanTypeEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CleanTypeEnum::Cache => write!(f, "cache"),
            CleanTypeEnum::Log => write!(f, "log"),
            CleanTypeEnum::All => write!(f, "all"),
        }
    }
}
