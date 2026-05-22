use clap::Parser;

mod action;
mod app;
mod config;

fn main() {
    let args = app::arg::Argument::parse();

    let config_path = args.config_path();

    match &args.action {
        app::arg::Action::Config | app::arg::Action::Version => {
            if let Err(e) = app::run_without_config(args.action) {
                log::error!("{}", e);
                std::process::exit(2);
            }
        }
        _ => match config::init(config_path) {
            Ok(config) => {
                if let Err(e) = app::run(&config) {
                    log::error!("{}", e);
                    std::process::exit(2);
                }
            }
            Err(e) => {
                log::error!("{}", e);
                std::process::exit(1);
            }
        },
    }
}
