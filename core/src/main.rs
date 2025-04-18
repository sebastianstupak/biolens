use biolens_core::cli;
use biolens_core::logging;

fn main() {
    let app = cli::build_cli();
    let matches = app.get_matches();

    let verbose = matches.get_flag("verbose");
    logging::setup_logger(verbose);

    cli::dispatch_command(matches);
}
