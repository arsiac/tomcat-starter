mod clean;
pub mod list;
pub mod run;

use crate::argument::{TmsAction, TmsArgument};
use crate::config::TmsConfiguration;

pub use crate::action::clean::CleanAction;
pub use crate::action::list::ListAction;
pub use crate::action::run::RunAction;

pub trait Actions {
    fn process(&self) -> bool;
}

pub fn execute(argument: Box<TmsArgument>, configuration: Box<TmsConfiguration>) -> bool {
    match argument.action {
        TmsAction::Run(arg) => RunAction::new(Box::new(arg), configuration).process(),
        TmsAction::Clean(arg) => CleanAction::new(Box::new(arg), configuration).process(),
        TmsAction::List(arg) => ListAction::new(Box::new(arg), configuration).process(),
    }
}
