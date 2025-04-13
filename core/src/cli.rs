use crate::commands::bamcov;
use crate::commands::common::CommandInfo;
use crate::commands::seqcomp;
use crate::commands::validate;
use clap::{ArgMatches, Command};
use std::collections::HashMap;

struct CommandModule {
    definition_fn: fn() -> CommandInfo,
    command_fn: fn() -> Command,
}

fn get_command_modules() -> Vec<CommandModule> {
    vec![
        CommandModule {
            definition_fn: validate::definition::command_info,
            command_fn: validate::definition::command,
        },
        CommandModule {
            definition_fn: bamcov::definition::command_info,
            command_fn: bamcov::definition::command,
        },
        CommandModule {
            definition_fn: seqcomp::definition::command_info,
            command_fn: seqcomp::definition::command,
        },
    ]
}

fn get_commands() -> Vec<CommandInfo> {
    get_command_modules()
        .iter()
        .map(|module| (module.definition_fn)())
        .collect()
}

pub fn build_cli() -> Command {
    let mut app = Command::new("biolens")
        .version("0.0.1")
        .author("Sebastián Stupák")
        .about("Bioinformatics analysis CLI tool");

    for module in get_command_modules() {
        app = app.subcommand((module.command_fn)());
    }

    app
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
