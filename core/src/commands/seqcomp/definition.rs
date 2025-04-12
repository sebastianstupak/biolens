use clap::{Command, Arg, ArgMatches};
use super::super::common::{CommandInfo, viz_arg, with_visualization};
use super::{META, handler};

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
            Arg::new("input1")
                .short('a')
                .long("input1")
                .required(true)
                .help("First sequence file (FASTA)"),
        )
        .arg(
            Arg::new("input2")
                .short('b')
                .long("input2")
                .required(true)
                .help("Second sequence file (FASTA)"),
        )
        .arg(viz_arg())
}

pub fn handle(matches: &ArgMatches) {
    let input1 = matches.get_one::<String>("input1").unwrap();
    let input2 = matches.get_one::<String>("input2").unwrap();

    let result = handler::compare_sequences(input1, input2);

    if matches.get_flag("visualize") {
        with_visualization(handler::visualize_comparison, &result);
    }
}
