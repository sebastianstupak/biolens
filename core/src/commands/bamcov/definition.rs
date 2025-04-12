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
            Arg::new("input")
                .short('i')
                .long("input")
                .required(true)
                .help("Input BAM file"),
        )
        .arg(
            Arg::new("region")
                .short('r')
                .long("region")
                .help("Genomic region (chr:start-end)"),
        )
        .arg(viz_arg())
}

pub fn handle(matches: &ArgMatches) {
    let input_file = matches.get_one::<String>("input").unwrap();
    let region = matches.get_one::<String>("region").map(|s| s.as_str());

    let result = handler::calculate_coverage(input_file, region);

    if matches.get_flag("visualize") {
        with_visualization(handler::visualize_coverage, &result);
    }
}
