use crate::commands::bamcov;
use crate::commands::common::CommandInfo;
use crate::commands::seqcomp;
use crate::commands::validate;
use clap::{Arg, ArgMatches, Command};
use log::{debug, info};
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
        .about("Bioinformatics analysis CLI tool")
        // Add global verbose flag that applies to all commands
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output")
                .global(true)
                .action(clap::ArgAction::SetTrue),
        );

    for module in get_command_modules() {
        app = app.subcommand((module.command_fn)());
    }

    app
}

pub fn dispatch_command(matches: ArgMatches) {
    let verbose = matches.get_flag("verbose");
    debug!("Dispatching command with verbose={}", verbose);

    let command_map = build_command_map();
    match matches.subcommand() {
        Some((cmd_name, sub_matches)) => {
            if let Some(handler) = command_map.get(cmd_name) {
                debug!("Executing command: {}", cmd_name);
                handler(sub_matches);
            } else {
                info!(
                    "Unknown command: {}. Use --help to see available commands.",
                    cmd_name
                );
            }
        }
        None => {
            info!("No command specified. Use --help to see available commands.");
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
