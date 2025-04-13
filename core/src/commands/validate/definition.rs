use super::super::common::CommandInfo;
use super::{handler, META};
use clap::{Arg, ArgMatches, Command};

pub fn command_info() -> CommandInfo {
    CommandInfo {
        name: META.name,
        aliases: META.aliases.to_vec(),
        handler: handle,
    }
}

pub fn command() -> Command {
    Command::new(META.name)
        .aliases(META.aliases)
        .about(META.about)
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .required(true)
                .help("Input file"),
        )
}

pub fn handle(matches: &ArgMatches) {
    let input_file = matches.get_one::<String>("input").unwrap();

    handler::validate(input_file);
}
