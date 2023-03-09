mod action;
mod argument;
mod config;
mod env_ini;
mod tomcat;
mod util;

use crate::config::configuration::TmsConfiguration;
use argument::TmsArgument;
use clap::Parser;
use log::error;
use std::process::exit;

fn main() {
    let args = Box::new(TmsArgument::parse());
    let config = match TmsConfiguration::load() {
        Ok(v) => v,
        Err(msg) => {
            error!("Load configuration failed: {}", msg);
            exit(1);
        }
    };

    if !action::execute(args, config) {
        exit(1);
    }
}
