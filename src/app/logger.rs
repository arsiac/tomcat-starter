use log::LevelFilter;
use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode};

pub fn init(log_level: LevelFilter) {
    let log_config = ConfigBuilder::new()
        .set_location_level(LevelFilter::Off)
        .set_target_level(LevelFilter::Off)
        .set_time_level(LevelFilter::Off)
        .set_thread_level(LevelFilter::Off)
        .build();

    let init_result = TermLogger::init(
        log_level,
        log_config,
        TerminalMode::Mixed,
        ColorChoice::Auto,
    );
    if let Err(e) = init_result {
        panic!("Failed to init logger: {}", e)
    }
}
