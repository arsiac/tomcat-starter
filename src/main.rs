mod action;
mod app;
mod config;

fn main() {
    match config::init() {
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
    }
}
