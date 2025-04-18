use biolens_core::cli;
use biolens_core::logging;
use tracing::debug;

fn main() {
    let app = cli::build_cli();
    let matches = app.get_matches();

    let verbosity = matches.get_count("verbose") as u8;
    let log_to_file = matches.get_flag("log-file");
    let json_format = matches.get_flag("json");

    let config = logging::load_config(Some(verbosity), Some(log_to_file), Some(json_format));

    if let Err(err) = logging::setup_logger(config.clone()) {
        eprintln!("Failed to initialize logger: {}", err);
        let basic_config = logging::LogConfig {
            verbosity,
            log_to_file: false,
            json_format: false,
            ..config
        };
        if let Err(fallback_err) = logging::setup_logger(basic_config) {
            eprintln!(
                "Critical error: Failed to initialize fallback logger: {}",
                fallback_err
            );
            return;
        }
    }

    debug!("Debug logging enabled");
    debug!("BioLens starting up");

    cli::dispatch_command(matches);

    debug!("BioLens shutting down");
}
