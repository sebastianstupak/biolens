use crate::commands::bamcov;
use crate::commands::common::CommandInfo;
use crate::commands::seqcomp;
use clap::{ArgMatches, Command};
use std::collections::HashMap;

pub fn build_cli() -> Command {
    let mut app = Command::new("biolens")
        .version("0.0.1")
        .author("Sebastián Stupák")
        .about("Bioinformatics analysis CLI tool");

    app = app.subcommand(bamcov::definition::command());
    app = app.subcommand(seqcomp::definition::command());

    app
}

fn get_commands() -> Vec<CommandInfo> {
    vec![
        bamcov::definition::command_info(),
        seqcomp::definition::command_info(),
    ]
}

pub fn dispatch_command(matches: ArgMatches) {
    let command_map = build_command_map();

    match matches.subcommand() {
        Some((cmd_name, sub_matches)) => {
            if let Some(handler) = command_map.get(cmd_name) {
                handler(sub_matches);
            } else {
                println!(
                    "Unknown command: {}. Use --help to see available commands.",
                    cmd_name
                );
            }
        }
        None => {
            println!("No command specified. Use --help to see available commands.");
        }
    }
}

fn build_command_map() -> HashMap<&'static str, fn(&ArgMatches)> {
    let mut map = HashMap::new();

    for cmd_info in get_commands() {
        map.insert(cmd_info.name, cmd_info.handler);

        for alias in cmd_info.aliases {
            map.insert(alias, cmd_info.handler);
        }
    }

    map
}
