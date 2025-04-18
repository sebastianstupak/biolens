use log::{debug, LevelFilter};

pub fn setup_logger(verbose: bool) {
    let level = if verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    env_logger::Builder::new()
        .filter_level(level)
        .format_timestamp(None)
        .init();

    debug!("Logger initialized with verbose={}", verbose);
}
