use biolens_core::cli;

fn main() {
    let app = cli::build_cli();
    let matches = app.get_matches();

    cli::dispatch_command(matches);
}
