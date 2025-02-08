use log::LevelFilter;
use simplelog::{TermLogger, Config, TerminalMode, ColorChoice};

pub fn configure_logs() {
    TermLogger::init(
        LevelFilter::Trace,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto
    ).unwrap();
}
